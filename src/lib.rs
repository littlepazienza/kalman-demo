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
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
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
        let width = 640;
        let height = 640;
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

    /*
     * Used for test.
     */
    pub fn set_kalman_goal(&mut self, goal_x: f32, goal_y: f32) {
        if goal_x > 0f32 && goal_x < self.width as f32 && goal_y > 0f32 && goal_y < self.height as f32 {
            self.kalman.set_goal(goal_x, goal_y)
        } else {
            self.kalman.set_goal(-1f32, -1f32);
            log(&format!("Cannot set goal outside of universe bounds of ([0-{}], [0-{}])", self.width, self.height));
        }
    }
}