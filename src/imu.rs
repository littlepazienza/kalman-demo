use std::ops::Mul;
use statrs::distribution::{MultivariateNormal};
use nalgebra::{DVector, DMatrix, VecStorage, Matrix, Dyn};
use rand::distributions::Distribution;
use statrs::statistics::{MeanN, VarianceN};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Imu {
    /*
     * The emulated real position of kalman.
     */
    actual_pos: Vec<f64>,

    /*
     * The bivariate distribution of error for reading the rotational angle and velocity of kalman.
     */
    distribution: Box<MultivariateNormal>,
}

impl Imu {
    pub fn new(x: f64, y: f64) -> Imu {
        Imu {
            actual_pos: Vec::from([x, y, 0.0, 0.0]),
            distribution: Box::new(MultivariateNormal::new(vec![0., 0.], vec![0.001, 0.1, 0.1, 0.001]).unwrap()),
        }
    }

    pub fn get_actual_pos(&self) -> Vec<f64> {
        return self.actual_pos.clone();
    }

    pub unsafe fn get_error_mean(&self) -> DVector<f64> {
        log(&format!("Movement error mean is:{:?})", (*self.distribution).mean()));
        (*self.distribution).mean().unwrap()
    }

    pub unsafe fn get_error_cov(&self) -> DMatrix<f64> {
        log(&format!("Movement error covariance is:{:?})", (*self.distribution).variance()));
        (*self.distribution).variance().unwrap()
    }

    pub fn set_error(&mut self, m: Vec<f64>, covariance_matrix: Vec<f64>) {
        log(&format!("Setting my rotation error to N({:?}, {:?})", m, covariance_matrix));
        *self.distribution = MultivariateNormal::new(m, covariance_matrix).unwrap()
    }

    pub unsafe fn simulate(&mut self, velocity: f64, theta: f64, universe: Universe) {
        log(&format!("Simulating 1 ms with rotation {}, velocity {}, position {}, {}", theta, velocity, self.actual_pos[0], self.actual_pos[1]));

        let sample = (*self.distribution).sample(&mut rand::thread_rng());
        let vel_error = sample[0];
        let rot_error = sample[1];

        // Update the velocity
        self.actual_pos[2] = velocity + velocity * vel_error;
        log(&format!("Commanded velocity: {}, actual: {}", velocity, self.actual_pos[2]));

        // Update the rotation
        self.actual_pos[3] = theta + theta * rot_error;
        log(&format!("Commanded rotation: {}, actual: {}", theta, self.actual_pos[3]));

        // If movement is expected, introduce movement variability from unknown variables
        if velocity != 0. {
            // Update the x position
            self.actual_pos[0] = (self.actual_pos[0] + self.actual_pos[3].cos() * self.actual_pos[2]).max(0.).min(universe.width as f64);
            // Update the y position
            self.actual_pos[1] = (self.actual_pos[1] + self.actual_pos[3].sin() * self.actual_pos[2]).max(0.).min(universe.height as f64);
        }
    }

    /*
     * Read the actual velocity achieved by kalman, plus some error.
     */
    pub unsafe fn read_imu(&self) -> ImuRead {
        let sample = (*self.distribution).sample(&mut rand::thread_rng());
        let vel_error = sample[0];
        let rot_error = sample[1];
        let velocity = self.actual_pos[2] + self.actual_pos[2] * vel_error;
        let rotation = self.actual_pos[3] + self.actual_pos[3] * rot_error;
        let matrix: DMatrix<f64> = DMatrix::from_vec(2, 2, vec![velocity, 1.0, 1.0, rotation]);
        let matrix1: Matrix<f64, Dyn, Dyn, VecStorage<f64, Dyn, Dyn>> = (*self.distribution).variance().unwrap();
        ImuRead {
            state: DVector::from_vec(vec![velocity, rotation]),
            covariance_matrix: matrix * matrix1
        }
    }
 }

#[wasm_bindgen]
#[derive(Clone)]
pub struct ImuRead {
    state: DVector<f64>,
    covariance_matrix: DMatrix<f64>
}

impl ImuRead {
    pub fn get_state(&self) -> DVector<f64> {
        self.state.clone()
    }

    pub fn get_covariance_matrix(&self) -> DMatrix<f64> {
        self.covariance_matrix.clone()
    }
}