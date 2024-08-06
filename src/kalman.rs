use std::vec::Vec;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::Universe;

static VELOCITY: f32 = 0.3f32;

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
            pos: Vec::from([seed_w, seed_h, VELOCITY, 0.0]),
            goal: Vec::from([seed_w, seed_h])
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
        if self.pos[2] > 0f32 {
            self.pos[0] = (self.pos[0] + self.pos[3].cos() * self.pos[2]).max(0f32).min(universe.width as f32);
            self.pos[1] = (self.pos[1] + self.pos[3].sin() * self.pos[2]).max(0f32).min(universe.height as f32);
        }
    }

    /*
     * Read from the rotation sensor which has a reading accuracy of 98%.
     * Kalman rotates with 98% accuracy, meaning he rotates exactly as commanded 98% of the time
     */
    pub fn read_rotation_sensor() {

    }
}
