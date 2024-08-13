use ndarray::{arr2, Array2};
use rand::distributions::Distribution;
use rand_distr::Normal;
use rand_distr::num_traits::Pow;
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
     * The commanded velocity and rotation of kalman.
     */
    commands: Vec<f32>,

    /*
     * The distribution of error for reading the rotational angle of kalman.
     */
    rotation_distribution: Box<Normal<f32>>,

    /*
     * The distribution of error for reading the velocity of kalman.
     */
    velocity_distribution: Box<Normal<f32>>,

    /*
     * The distribution of error for reading the position of kalman.
     */
    position_distribution: Box<Normal<f32>>
}

impl Imu {
    pub fn new(x: f32, y: f32) -> Imu {
        Imu {
            actual_pos: Vec::from([x, y, 0.0, 0.0]),
            commands: Vec::from([0.0, 0.0]),
            rotation_distribution: Box::new(Normal::new(0.0, 0.001).unwrap()),
            velocity_distribution: Box::new(Normal::new(0.0, 0.001).unwrap()),
            position_distribution: Box::new(Normal::new(0.0, 0.001).unwrap())
        }
    }

    pub fn get_actual_pos(&self) -> Vec<f32> {
        return self.actual_pos.clone();
    }

    pub unsafe fn get_movement_error(&self) -> Vec<f32> {
        log(&format!("Movement error is N({}, {})", (*self.velocity_distribution).mean(), (*self.velocity_distribution).std_dev()));
        Vec::from([(*self.velocity_distribution).mean(), (*self.velocity_distribution).std_dev()])
    }

    pub unsafe fn get_rotation_error(&self) -> Vec<f32> {
        log(&format!("Rotation error is N({}, {})", (*self.rotation_distribution).mean(), (*self.rotation_distribution).std_dev()));
        Vec::from([(*self.rotation_distribution).mean(), (*self.rotation_distribution).std_dev()])
    }

    pub unsafe fn get_position_error(&self) -> Vec<f32> {
        log(&format!("Rotation error is N({}, {})", (*self.position_distribution).mean(), (*self.position_distribution).std_dev()));
        Vec::from([(*self.position_distribution).mean(), (*self.position_distribution).std_dev()])
    }


    pub unsafe fn set_command_rotation(&mut self, theta: f32) {
        self.commands[1] = theta;
    }

    pub unsafe fn set_command_velocity(&mut self, velocity: f32) {
        self.commands[0] = velocity;
    }

    pub fn set_movement_error(&mut self, m: f32, std: f32) {
        log(&format!("Setting my rotation error to N({}, {})", m, std));
        *self.velocity_distribution = Normal::new(m, std).unwrap();
    }

    pub fn set_rotation_error(&mut self, m: f32, std: f32) {
        log(&format!("Setting my rotation error to N({}, {})", m, std));
        *self.rotation_distribution = Normal::new(m, std).unwrap();
    }

    pub fn set_position_error(&mut self, m: f32, std: f32) {
        log(&format!("Setting my position error to N({}, {})", m, std));
        *self.position_distribution = Normal::new(m, std).unwrap();
    }

    pub unsafe fn simulate(&mut self, universe: Universe) {
        log(&format!("Simulating 1 ms with rotation {}, velocity {}, position {}, {}", self.commands[1], self.commands[0], self.actual_pos[0], self.actual_pos[1]));

        // Update the velocity
        self.actual_pos[2] = self.generate_velocity_with_error(self.commands[0]);
        log(&format!("Commanded velocity: {}, actual: {}", self.commands[0], self.actual_pos[2]));

        // Update the rotation
        self.actual_pos[3] = self.generate_rotation_with_error(self.commands[1]);
        log(&format!("Commanded rotation: {}, actual: {}", self.commands[1], self.actual_pos[3]));

        // If movement is expected, introduce movement variability from unknown variables
        if self.commands[0] != 0f32 {
            // Update the x position
            let pre_x = (self.actual_pos[0] + self.actual_pos[3].cos() * self.actual_pos[2]).max(0f32).min(universe.width as f32);
            self.actual_pos[0] = self.generate_position_with_error(pre_x);
            log(&format!("Expected x: {}, actual: {}", pre_x, self.actual_pos[0]));

            // Update the y position
            let pre_y = (self.actual_pos[1] + self.actual_pos[3].sin() * self.actual_pos[2]).max(0f32).min(universe.height as f32);
            self.actual_pos[1] = self.generate_position_with_error(pre_y);
            log(&format!("Expected y: {}, actual: {}", pre_y, self.actual_pos[1]));
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
                [velocity * (*self.velocity_distribution).std_dev().pow(2), 0.0],
                [0.0, rotation * (*self.rotation_distribution).std_dev().pow(2)]
            ])
        }
    }
 }

impl Imu {

    /*
     * Expected distance plus an error generated by the error distribution.
     */
    unsafe fn generate_velocity_with_error(&self, expected_dist: f32) -> f32 {
        expected_dist + (expected_dist * (*self.velocity_distribution).sample(&mut rand::thread_rng()))
    }

    /*
     * Expected rotation plus an error generated by the error distribution.
     */
    unsafe fn generate_rotation_with_error(&self, expected_rotation: f32) -> f32 {
        expected_rotation + (expected_rotation * (*self.rotation_distribution).sample(&mut rand::thread_rng()))
    }

    /*
     * Expected rotation plus an error generated by the error distribution.
     */
    unsafe fn generate_position_with_error(&self, expected_pos: f32) -> f32 {
        expected_pos + (expected_pos * (*self.position_distribution).sample(&mut rand::thread_rng()))
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