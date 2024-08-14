use std::ops::{Add, Index, Mul, Sub};
use std::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe, imu::Imu};
use ndarray::{arr1, arr2, Array2};

static VELOCITY: f32 = 1f32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    /*
     * The believed position of kalman.
     */
    belief_pos: Vec<f32>,

    /*
     * The believed state of kalman.
     */
    state: Vec<f32>,

    /*
     * The accrued covariance matrix
     */
    covariance_matrix: Box<Array2<f32>>,

    /*
     * The goal x and y.
     */
    goal: Vec<f32>,

    /*
     * The emulated inertial measurement unit (IMU).
     */
    imu: Imu,
}

#[wasm_bindgen]
impl Kalman {
    pub fn get_goal(&self) -> Vec<f32> {
        return self.goal.clone()
    }

    pub fn get_belief(&self) -> Vec<f32> {
        Vec::from([self.belief_pos[0], self.belief_pos[1], self.state[0], self.state[1]])
    }

    pub fn get_actual(&self) -> Vec<f32> {
        return self.imu.get_actual_pos();
    }

    pub fn at_goal(&self) -> bool {
        self.goal[0] < self.belief_pos[0] && self.belief_pos[0] < self.goal[0] + 10f32
            && self.goal[1] < self.belief_pos[1] && self.belief_pos[1] < self.goal[1] + 10f32
    }

    pub unsafe fn get_movement_error(&self) -> Vec<f32> {
        self.imu.get_movement_error()
    }

    pub unsafe fn get_rotation_error(&self) -> Vec<f32> {
        self.imu.get_rotation_error()
    }

    pub unsafe fn get_position_error(&self) -> Vec<f32> {
        self.imu.get_position_error()
    }
}

impl Kalman {

    pub fn new(x: f32, y: f32) -> Kalman {
        Kalman {
            belief_pos: Vec::from([x, y]),
            state: Vec::from([0.0, 0.0]),
            covariance_matrix: Box::new(arr2(&[
                [1.0, 0.0],
                [0.0, 1.0]
            ])),
            goal: Vec::from([-1f32, -1f32]),
            imu: Imu::new(x, y),
        }
    }

    /*
     * Update 1 ms of movement.
     */
    pub unsafe fn tick(&mut self, universe: Universe) {
        let mut velocity: f32 = self.state[0];
        let mut theta: f32 = self.state[1];
        if self.at_goal() {
            velocity = 0f32;
        } else {
            // Rotate to correct based on the belief, and move at a fixed speed
            theta = self.calculate_rotation_to_goal();
            velocity = VELOCITY;
        }

        // Let the simulated IMU update
        self.imu.simulate(velocity, theta, universe.clone());

        // Execute the kalman filter
        // Step one predict the future state and state covariance matrix before measurement
        if velocity != 0f32 {
            let x1 = arr2(&[
                [velocity, theta]
            ]);
            let z = arr2(&[
                [self.state[0] - velocity, self.state[1] - theta]
            ]);
            let Q = arr2(&[
                [0.01, 0.001],
                [0.001, 0.01]
            ]);
            let R = arr2(&[
                [0.01, 0.001],
                [0.001, 0.01]
            ]);
            let I = arr2(&[
                [1.0, 0.0],
                [0.0, 1.0]
            ]);
            let P1 = (*self.covariance_matrix).clone().add(Q.clone());

            // Take a measurement, save off the measurement covariance matrix
            let read = self.imu.read_imu();
            let H = read.get_covariance_matrix();

            // Step 2, calculate the kalman gain, new adjusted state and state covariance matrix
            let K = P1.clone() * H.t() * arr2(&[[1.0, 1.0], [1.0, 1.0]]) / (H.clone() * P1.clone() * H.t() + R.t());
            let x = x1.clone().add(K.clone().mul(z.sub(H.clone().mul(x1.clone()))));
            let P = I.sub(K.clone().mul(H.clone())).mul(P1.clone());
            self.state[0] = *x.index([0, 0]);
            self.state[1] = *x.index([0, 1]);
            *self.covariance_matrix = P;

            // Update the beliefs of the x and y position using our new adjusted state
            self.belief_pos[0] = (self.belief_pos[0] + self.state[1].cos() * self.state[0]).max(0f32).min(universe.width as f32);
            self.belief_pos[1] = (self.belief_pos[1] + self.state[1].sin() * self.state[0]).max(0f32).min(universe.height as f32);

            log(&format!("Kalman filter complete. x: {}, y: {}, velocity: {}, rotation: {}, state_covariance: {}", self.belief_pos[0], self.belief_pos[1], self.state[0], self.state[1], self.covariance_matrix.to_string()));
        }
    }

    /*
     * Calculate the rotation required to get to the goal.
     */
    fn calculate_rotation_to_goal(&self) -> f32 {
        let delta_x = self.goal[0] - self.belief_pos[0];
        let delta_y = self.goal[1] - self.belief_pos[1];
        let theta = (delta_y.abs() / delta_x.abs()).atan();

        // Determine the quadrant and apply the correct reference angle
        if delta_x.is_sign_positive() && delta_y.is_sign_positive() {
            theta
        } else if !delta_x.is_sign_positive() && delta_y.is_sign_positive() {
            180f32.to_radians() - theta
        } else if !delta_x.is_sign_positive() && !delta_y.is_sign_positive() {
            theta - 180f32.to_radians()
        } else if delta_x.is_sign_positive() && !delta_y.is_sign_positive() {
            360f32.to_radians() - theta
        } else {
            0f32
        }
    }

    /*
     * Sets a goal.
     */
    pub fn set_goal(&mut self, goal_x: f32, goal_y: f32) {
        self.goal[0] = goal_x;
        self.goal[1] = goal_y;
    }

    /*
     * Set the rotation error, used in test.
     */
    pub unsafe fn set_rotation_error(&mut self, m: f32, std: f32) {
        self.imu.set_rotation_error(m, std);
    }

    /*
     * Set the movement error, used in test.
     */
    pub unsafe fn set_movement_error(&mut self, m: f32, std: f32) {
        self.imu.set_movement_error(m, std);
    }

    /*
     * Set the position error, used in test.
     */
    pub unsafe fn set_position_error(&mut self, m: f32, std: f32) {
        self.imu.set_position_error(m, std);
    }
}
