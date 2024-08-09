use std::ops::{Deref, DerefMut};
use std::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{log, Universe};
use rand_distr::{Normal, Distribution};

static VELOCITY: f32 = 1f32;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    belief_pos: Vec<f32>,
    actual_pos: Vec<f32>,
    goal: Vec<f32>,
    rotation_distr: Box<Normal<f32>>,
    movement_distr: Box<Normal<f32>>
}

#[wasm_bindgen]
impl Kalman {
    pub fn get_x(&self) -> f32 {
        return self.actual_pos[0];
    }

    pub fn get_y(&self) -> f32 {
        return self.actual_pos[1];
    }

    pub fn get_velocity(&self) -> f32 {
        return self.actual_pos[2];
    }

    pub fn get_rotation(&self) -> f32 {
        return self.actual_pos[3];
    }

    pub fn get_goal(&self) -> Vec<f32> {
        return self.goal.clone()
    }

    pub fn get_belief(&self) -> Vec<f32> {
        return self.belief_pos.clone();
    }

    pub fn get_actual(&self) -> Vec<f32> {
        return self.actual_pos.clone();
    }

    pub fn at_goal(&self) -> bool {
        self.goal[0] < self.belief_pos[0] && self.belief_pos[0] < self.goal[0] + 10f32
            && self.goal[1] < self.belief_pos[1] && self.belief_pos[1] < self.goal[1] + 10f32
    }

    pub unsafe fn get_movement_error(&self) -> Vec<f32> {
        log(&format!("Movement error is N({}, {})", (*self.movement_distr).mean(), (*self.movement_distr).std_dev()));
        Vec::from([(*self.movement_distr).mean(), (*self.movement_distr).std_dev()])
    }

    pub unsafe fn get_rotation_error(&self) -> Vec<f32> {
        log(&format!("Rotation error is N({}, {})", (*self.rotation_distr).mean(), (*self.rotation_distr).std_dev()));
        Vec::from([(*self.rotation_distr).mean(), (*self.rotation_distr).std_dev()])
    }
}

impl Kalman {

    pub fn new(seed_w: f32, seed_h: f32) -> Kalman {
        Kalman {
            actual_pos: Vec::from([seed_w, seed_h, 0.0, 0.0]),
            belief_pos: Vec::from([seed_w, seed_h, 0.0, 0.0]),
            goal: Vec::from([-1f32, -1f32]),
            rotation_distr: Box::new(Normal::new(0.0, 0.001).unwrap()),
            movement_distr: Box::new(Normal::new(0.0, 0.001).unwrap())
        }
    }

    /*
      * Command the rotation plc to rotate to a given rotation in radians.
      */
    pub unsafe fn command_rotation(&mut self, rotation: f32) {
        self.belief_pos[3] = rotation;
        self.actual_pos[3] = self.generate_real_rotation(rotation);
        log(&format!("Commanded rotation: {}, actual: {}", self.belief_pos[3], self.actual_pos[3]));
    }

    /*
     * Command the movement plc to move the bot at a given velocity.
     */
    pub unsafe fn command_movement(&mut self, velocity: f32) {
        self.belief_pos[2] = velocity;
        self.actual_pos[2] = self.generate_real_distance(velocity);
        log(&format!("Commanded velocity: {}, actual: {}", self.belief_pos[2], self.actual_pos[2]));
    }
    /*
     * Execute 1 ms of movement.
     */
    pub fn execute_movement(&mut self, universe: Universe) {
        // Update the believed position based off the registered believed current position, velocity and rotation
        self.belief_pos[0] = (self.belief_pos[0] + self.belief_pos[3].cos() * self.belief_pos[2]).max(0f32).min(universe.width as f32);
        self.belief_pos[1] = (self.belief_pos[1] + self.belief_pos[3].sin() * self.belief_pos[2]).max(0f32).min(universe.height as f32);

        // Update the actual position based off the registered actual current position, velocity and rotation
        self.actual_pos[0] = (self.actual_pos[0] + self.actual_pos[3].cos() * self.actual_pos[2]).max(0f32).min(universe.width as f32);
        self.actual_pos[1] = (self.actual_pos[1] + self.actual_pos[3].sin() * self.actual_pos[2]).max(0f32).min(universe.height as f32);
    }

    /*
     * Update 1 ms of movement.
     */
    pub unsafe fn tick(&mut self, universe: Universe) {
        if !self.at_goal() {
            // If the bot's angle is wrong, it will rotate to correct, otherwise it will move
            let desired_theta = self.calculate_rotation_to_goal();
            if self.belief_pos[3] != desired_theta {
                self.command_rotation(desired_theta);
            } else if self.belief_pos[2] != VELOCITY {
                self.command_movement(VELOCITY);
                self.execute_movement(universe);
            } else {
                self.execute_movement(universe);
            }
        } else {
            self.command_rotation(0f32);
            self.command_movement(0f32);
        }
    }

    /*
     * Calculate the rotation required to get to the goal.
     */
    fn calculate_rotation_to_goal(&self) -> f32 {
        let delta_x = self.goal[0] - self.belief_pos[0];
        let delta_y = self.goal[1] - self.belief_pos[1];
        let theta = (delta_y / delta_x).atan();

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
     * Expected distance plus an error generated by the error distribution.
     */
    unsafe fn generate_real_distance(&self, expected_dist: f32) -> f32 {
        expected_dist + (expected_dist * (*self.movement_distr).sample(&mut rand::thread_rng()))
    }

    /*
     * Expected rotation plus an error generated by the error distribution.
     */
    unsafe fn generate_real_rotation(&self, expected_rotation: f32) -> f32 {
        expected_rotation + (expected_rotation * (*self.rotation_distr).sample(&mut rand::thread_rng()))
    }

    /*
     * Set the rotation error, used in test.
     */
    pub unsafe fn set_rotation_error(&mut self, m: f32, std: f32) {
        log(&format!("Setting my rotation error to N({}, {})", m, std));
        *self.rotation_distr = Normal::new(m, std).unwrap();
    }

    /*
     * Set the movement error, used in test.
     */
    pub unsafe fn set_movement_error(&mut self, m: f32, std: f32) {
        log(&format!("Setting my rotation error to N({}, {})", m, std));
        *self.movement_distr = Normal::new(m, std).unwrap();
    }
}
