use std::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::Universe;

static VELOCITY: f32 = 1f32;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    pos: Vec<f32>,
    goal: Vec<f32>
}

#[wasm_bindgen]
impl Kalman {
    pub fn get_x(&self) -> f32 {
        return self.pos[0];
    }

    pub fn get_y(&self) -> f32 {
        return self.pos[1];
    }

    pub fn get_velocity(&self) -> f32 {
        return self.pos[2];
    }

    pub fn get_rotation(&self) -> f32 {
        return self.pos[3];
    }
}


impl Kalman {

    pub fn new(seed_w: f32, seed_h: f32) -> Kalman {
        Kalman {
            pos: Vec::from([seed_w, seed_h, 0.0, 0.0]),
            goal: Vec::from([-1f32, -1f32])
        }
    }

    pub fn set_velocity(&mut self, velocity: f32) {
        self.pos[2] = velocity;
    }

    pub fn set_rotation(&mut self, rotation: f32) {
        self.pos[3] = rotation;
    }

    /*
     * Update 1 ms of movement.
     */
    pub fn tick(&mut self, universe: Universe) {
        if self.goal[0] == self.pos[0] && self.goal[1] == self.pos[1] {
            self.pos[2] = 0f32;
        } else if self.goal[0] > 0f32 && self.goal[1] > 0f32 {
            self.set_rotation(self.calculate_rotation_to_goal());
            self.pos[2] = VELOCITY;
            self.pos[0] = (self.pos[0] + self.pos[3].cos() * self.pos[2]).max(0f32).min(universe.width as f32);
            self.pos[1] = (self.pos[1] + self.pos[3].sin() * self.pos[2]).max(0f32).min(universe.height as f32);
            log(&format!("x = {}, y = {}, velocity = {}, theta = {}", self.pos[0], self.pos[1], self.pos[2], self.pos[3]));
        }
    }

    /*
     * Calculate the rotation required to get to the goal.
     */
    fn calculate_rotation_to_goal(&self) -> f32 {
        let delta_x = self.goal[0] - self.pos[0];
        let delta_y = self.goal[1] - self.pos[1];
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
     * Read from the rotation sensor which has a reading accuracy of 98%.
     * Kalman rotates with 98% accuracy, meaning he rotates exactly as commanded 98% of the time
     */
    pub fn read_rotation_sensor() {

    }
}
