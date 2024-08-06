extern crate core;

mod kalman;

use wasm_bindgen::prelude::*;
use kalman::Kalman;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u32,
    height: u32,
    kalman: Kalman,
}

#[wasm_bindgen]
impl Universe {
    /*
     * Execute a single timestep.
     */
    pub fn tick(&mut self) {
        self.kalman.tick(self.clone());
    }

    pub fn new(seed_w: f32, seed_h: f32) -> Universe {
        let width = 6400;
        let height = 6400;
        let kalman = Kalman::new(seed_w, seed_h);

        Universe {
            width,
            height,
            kalman,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn kalman(&self) -> Kalman {
        self.kalman.clone()
    }

    /*
     * Used for test.
     */
    pub fn set_kalman_velocity(&mut self, velocity: f32) {
        self.kalman.set_velocity(velocity)
    }

    /*
     * Used for test.
     */
    pub fn set_kalman_rotation(&mut self, rotation: f32) {
        self.kalman.set_rotation(rotation)
    }
}