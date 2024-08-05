use std::intrinsics::rotate_left;
use std::vec;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone)]
pub struct Kalman {
    pos: vec<f32>,
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

    pub fn set_velocity(&self, velocity: f32) {
        self.pos[2] = velocity;
    }

    pub fn set_rotation(&self, rotation: f32) {
        self.pos[3] = rotation;
    }

    /*
     * Update 1 ms.
     */
    pub fn tick(&self) {
        self.pos[0] = self.pos[2]
    }
}


impl Kalman {

    pub fn new(seed_w: f32, seed_h: f32) -> Kalman {
        Kalman {
            pos: [seed_w, seed_h, 0.0, 0.0]
        }
    }

    pub fn update_index(&mut self) {
        // Do something to the cells based on the decision of the agent.
        if self.get_y() < self.y {
            self.prev_col = self.y;
            self.prev_row = self.x;
            if (self.y < g.width) {
                self.y += 10;
            } else {
                self.prev_col = self.y;
                self.x -=10;
            }
        } else {
            self.prev_col = self.y;
            if (self.y > 0) {
                self.y -= 10;
            } else {
                self.y += 10;
            }
        }
    }

    /*
     * Read from the rotation sensor, 80% (TBD) of the time, the rotation is succesful and the bot
     */
    pub fn read_sensor() {

    }
}
