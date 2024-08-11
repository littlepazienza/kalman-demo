use std::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe, imu::Imu};
use ndarray::{arr2, Array2};

static VELOCITY: f32 = 1f32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    /*
     * The believed state of kalman.
     */
    belief_pos: Vec<f32>,

    /*
     * The accrued covariance matrix
     */
    covariance_matrix: Array2<f32>,

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
    pub fn get_x(&self) -> f32 {
        return self.imu.get_actual_pos()[0];
    }

    pub fn get_y(&self) -> f32 {
        return self.imu.get_actual_pos()[1];
    }

    pub fn get_velocity(&self) -> f32 {
        return self.imu.get_actual_pos()[2];
    }

    pub fn get_rotation(&self) -> f32 {
        return self.imu.get_actual_pos()[3];
    }

    pub fn get_goal(&self) -> Vec<f32> {
        return self.goal.clone()
    }

    pub fn get_belief(&self) -> Vec<f32> {
        return self.belief_pos.clone();
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
            belief_pos: Vec::from([x, y, 0.0, 0.0]),
            covariance_matrix: arr2(&[
                [0.0, 0.0],
                [0.0, 0.0]
            ]),
            goal: Vec::from([-1f32, -1f32]),
            imu: Imu::new(x, y)
        }
    }

    /*
      * Command the rotation plc to rotate to a given rotation in radians.
      */
    pub unsafe fn command_rotation(&mut self, rotation: f32) {
        self.belief_pos[3] = rotation;
        self.imu.set_command_rotation(rotation);
    }

    /*
     * Command the movement plc to move the bot at a given velocity.
     */
    pub unsafe fn command_movement(&mut self, velocity: f32) {
        self.belief_pos[2] = velocity;
        self.imu.set_command_velocity(velocity);
    }

    /*
     * Update 1 ms of movement.
     */
    pub unsafe fn tick(&mut self, universe: Universe) {
        if self.at_goal() {
            self.command_movement(0f32);
        } else {
            // If the bot's angle is wrong, it will rotate to correct, otherwise it will move
            let desired_theta = self.calculate_rotation_to_goal();
            if self.belief_pos[3] != desired_theta {
                self.command_rotation(desired_theta);
            }

            // Always move at a fixed speed
            self.command_movement(VELOCITY);
        }

        // Let the simulated IMU update kalman's vectors with some induced error given the commands
        self.imu.simulate(universe.clone());

        // Read from the IMU and set a belief for velocity and rotation
        // TODO: Interpret the imu and build a belief via a kalman filter
        let read = self.imu.read_imu();
        self.belief_pos[2] = read.get_state()[0];
        self.belief_pos[3] = read.get_state()[1];

        // Update the beliefs of the x and y position
        self.belief_pos[0] = (self.belief_pos[0] + self.belief_pos[3].cos() * self.belief_pos[2]).max(0f32).min(universe.width as f32);
        self.belief_pos[1] = (self.belief_pos[1] + self.belief_pos[3].sin() * self.belief_pos[2]).max(0f32).min(universe.height as f32);
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
