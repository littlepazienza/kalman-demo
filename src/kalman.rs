use std::ops::{Add, Mul, Sub};
use std::vec::Vec;
use nalgebra::{DMatrix, DVector};
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe, imu::Imu};

static VELOCITY: f64 = 1.;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    /*
     * The believed position of kalman.
     */
    belief_pos: Vec<f64>,

    /*
     * The believed state of kalman.
     */
    state: Vec<f64>,

    /*
     * The accrued covariance matrix
     */
    covariance_matrix: Box<DMatrix<f64>>,

    /*
     * The goal x and y.
     */
    goal: Vec<f64>,

    /*
     * The emulated inertial measurement unit (IMU).
     */
    imu: Imu,
}

#[wasm_bindgen]
impl Kalman {
    pub fn get_goal(&self) -> Vec<f64> {
        self.goal.clone()
    }

    pub fn get_belief(&self) -> Vec<f64> {
        Vec::from([self.belief_pos[0], self.belief_pos[1], self.state[0], self.state[1]])
    }

    pub fn get_actual(&self) -> Vec<f64> {
        self.imu.get_actual_pos()
    }

    pub fn at_goal(&self) -> bool {
        self.goal[0] < self.belief_pos[0] && self.belief_pos[0] < self.goal[0] + 10.
            && self.goal[1] < self.belief_pos[1] && self.belief_pos[1] < self.goal[1] + 10.
    }

    pub unsafe fn get_error_mean(&self) -> Vec<f64> {
        let mu = self.imu.get_error_mean();
        vec![mu[0], mu[1]]
    }

    pub unsafe fn get_error_covariance(&self) -> Vec<f64> {
        let cov = self.imu.get_error_cov();
        vec![cov[0], cov[1], cov[2], cov[3]]
    }
}

impl Kalman {

    pub fn new(x: f64, y: f64) -> Kalman {
        Kalman {
            belief_pos: Vec::from([x, y]),
            state: Vec::from([0.0, 0.0]),
            covariance_matrix: Box::new(DMatrix::from_vec(2, 2, vec![0.0, 0.0, 0.0, 0.0])),
            goal: Vec::from([-1., -1.]),
            imu: Imu::new(x, y),
        }
    }

    /*
     * Update 1 ms of movement.
     */
    pub unsafe fn tick(&mut self, universe: Universe) {
        let mut velocity = self.state[0];
        let mut theta = self.state[1];
        if self.at_goal() {
            velocity = 0.;
        } else {
            // Rotate to correct based on the belief, and move at a fixed speed
            theta = self.calculate_rotation_to_goal();
            velocity = VELOCITY;
        }

        // Let the simulated IMU update
        self.imu.simulate(velocity, theta, universe.clone());

        // Execute the kalman filter
        // Step one predict the future state and state covariance matrix before measurement
        if velocity != 0. {
            let x1 = DVector::from_vec(vec![velocity, theta]);
            let z = DVector::from_vec(vec![self.state[0] - velocity, self.state[1] - theta]);
            let Q = DMatrix::from_vec(2, 2, vec![0.01, 0.001, 0.001, 0.01]);
            let R = DMatrix::from_vec(2, 2, vec![0.01, 0.001, 0.001, 0.01]);
            let I = DMatrix::from_vec(2, 2, vec![1.0, 0.0, 0.0, 1.0]);
            let P1 = (*self.covariance_matrix).clone().add(Q.clone());

            // Take a measurement, save off the measurement covariance matrix
            let read = self.imu.read_imu();
            let H = read.get_covariance_matrix();

            // Step 2, calculate the kalman gain, new adjusted state and state covariance matrix
            let K = P1.clone() * H.transpose() * (DMatrix::from_vec(2, 2, vec![1.0, 1.0, 1.0, 1.0]).component_div(&(H.clone() * P1.clone() * H.transpose() + R.transpose())));
            let x = x1.clone().add(K.clone().mul(z.sub(H.clone().mul(x1.clone()))));
            let P = I.sub(K.clone().mul(H.clone())).mul(P1.clone());
            self.state[0] = x[0];
            self.state[1] = x[1];
            *self.covariance_matrix = P;

            // Update the beliefs of the x and y position using our new adjusted state
            self.belief_pos[0] = (self.belief_pos[0] + self.state[1].cos() * self.state[0]).max(0.).min(universe.width as f64);
            self.belief_pos[1] = (self.belief_pos[1] + self.state[1].sin() * self.state[0]).max(0.).min(universe.height as f64);

            log(&format!("Kalman filter complete. x: {}, y: {}, velocity: {}, rotation: {}, state_covariance: {}", self.belief_pos[0], self.belief_pos[1], self.state[0], self.state[1], self.covariance_matrix.to_string()));
        }
    }

    /*
     * Calculate the rotation required to get to the goal.
     */
    fn calculate_rotation_to_goal(&self) -> f64 {
        let delta_x = self.goal[0] - self.belief_pos[0];
        let delta_y = self.goal[1] - self.belief_pos[1];
        let theta = (delta_y.abs() / delta_x.abs()).atan();

        // Determine the quadrant and apply the correct reference angle
        if delta_x.is_sign_positive() && delta_y.is_sign_positive() {
            theta
        } else if !delta_x.is_sign_positive() && delta_y.is_sign_positive() {
            180f64.to_radians() - theta
        } else if !delta_x.is_sign_positive() && !delta_y.is_sign_positive() {
            theta - 180f64.to_radians()
        } else if delta_x.is_sign_positive() && !delta_y.is_sign_positive() {
            360f64.to_radians() - theta
        } else {
            0.
        }
    }

    /*
     * Sets a goal.
     */
    pub fn set_goal(&mut self, goal_x: f64, goal_y: f64) {
        self.goal[0] = goal_x;
        self.goal[1] = goal_y;
    }

    /*
     * Set the rotation error, used in test.
     */
    pub unsafe fn set_error(&mut self, m: Vec<f64>, covariance: Vec<f64>) {
        self.imu.set_error(m, covariance);
    }
}
