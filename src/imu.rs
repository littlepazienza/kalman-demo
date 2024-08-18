use ndarray::{arr2, Array2};
use rstat::Distribution;
use rstat::normal::{MvNormal, MvNormalParams};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe};

#[wasm_bindgen]
#[derive(Clone)]
pub struct Imu {
    /*
     * The emulated real position of kalman.
     */
    actual_pos: Vec<f32>,

    /*
     * The bivariate distribution of error for reading the rotational angle and velocity of kalman.
     */
    distribution: Box<MvNormal>,
}

impl Imu {
    pub fn new(x: f32, y: f32) -> Imu {
        let cov = arr2(&[
            [0.001, 0.1],
            [0.1, 0.001]
        ]);
        Imu {
            actual_pos: Vec::from([x, y, 0.0, 0.0]),
            distribution: Box::new(MvNormal::new(vec![0.0, 0.0], cov).unwrap()),
        }
    }

    pub fn get_actual_pos(&self) -> Vec<f32> {
        return self.actual_pos.clone();
    }

    pub unsafe fn get_error(&self) -> MvNormalParams {
        log(&format!("Movement error is: N({:?}, {:?})", (*self.distribution).params().mu, (*self.distribution).params().Sigma));
        (*self.distribution).params()
    }

    pub fn set_error(&mut self, m: Vec<f32>, covariance_matrix: Array2<f32>) {
        log(&format!("Setting my rotation error to N({:?}, {:?})", m, covariance_matrix));
        *self.distribution = MvNormal::new(m, covariance_matrix).unwrap()
    }

    pub unsafe fn simulate(&mut self, velocity: f32, theta: f32, universe: Universe) {
        log(&format!("Simulating 1 ms with rotation {}, velocity {}, position {}, {}", theta, velocity, self.actual_pos[0], self.actual_pos[1]));

        // Update the velocity
        self.actual_pos[2] = self.generate_velocity_with_error(velocity);
        log(&format!("Commanded velocity: {}, actual: {}", velocity, self.actual_pos[2]));

        // Update the rotation
        self.actual_pos[3] = self.generate_rotation_with_error(theta);
        log(&format!("Commanded rotation: {}, actual: {}", theta, self.actual_pos[3]));

        // If movement is expected, introduce movement variability from unknown variables
        if velocity != 0f32 {
            // Update the x position
            self.actual_pos[0] = (self.actual_pos[0] + self.actual_pos[3].cos() * self.actual_pos[2]).max(0f32).min(universe.width as f32);
            // Update the y position
            self.actual_pos[1] = (self.actual_pos[1] + self.actual_pos[3].sin() * self.actual_pos[2]).max(0f32).min(universe.height as f32);
        }
    }

    /*
     * Read the actual velocity achieved by kalman, plus some error.
     */
    pub unsafe fn read_imu(&self) -> ImuRead {
        let velocity = self.generate_velocity_with_error(self.actual_pos[2]);
        let rotation = self.generate_rotation_with_error(self.actual_pos[3]);
        ImuRead {
            state: Vec::from([velocity, rotation]),
            covariance_matrix: arr2(&[
                [velocity, 1.0],
                [1.0, rotation]
            ]) * self.distribution.params().Sigma
        }
    }
 }

#[wasm_bindgen]
#[derive(Clone)]
pub struct ImuRead {
    state: Vec<f32>,
    covariance_matrix: Array2<f32>
}

impl ImuRead {
    pub fn get_state(&self) -> Vec<f32> {
        self.state.clone()
    }

    pub fn get_covariance_matrix(&self) -> Array2<f32> {
        self.covariance_matrix.clone()
    }
}