extern crate core;

mod kalman;

use wasm_bindgen::prelude::*;
use kalman::Kalman;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Universe {
    width: u64,
    height: u64,
    kalman: Kalman,
}

#[wasm_bindgen]
impl Universe {
    /*
     * Static function for returning the 1D index from the 2D index.
     */
    pub fn get_index(width: u64, row: u64, column: u64) -> usize {
        (row * width + column) as usize
    }

    /*
     * Execute a single timestep.
     */
    pub fn tick(&mut self) {
        self.kalman.update_index();
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

    pub fn width(&self) -> u64 {
        self.width
    }

    pub fn height(&self) -> u64 {
        self.height
    }
}