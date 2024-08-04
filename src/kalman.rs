use crate::Graph;

#[derive(Clone)]
pub struct Kalman {
    pub(crate) row: u32,
    pub(crate) column: u32,
    prev_row: u32,
    prev_col: u32
}


impl Kalman {

    pub fn new(seed_w: u32, seed_h: u32) -> Kalman {
        Kalman {
            row: seed_w,
            column: seed_h,
            prev_row: seed_w,
            prev_col: seed_h
        }
    }

    pub fn update_index(&mut self, g: Graph) {
        // Do something to the cells based on the decision of the agent.
        if self.prev_col < self.column {
            self.prev_col = self.column;
            self.prev_row = self.row;
            if (self.column < g.width) {
                self.column += 1;
            } else {
                self.prev_col = self.column;
                self.column -=1;
            }
        } else {
            self.prev_col = self.column;
            if (self.column > 0) {
                self.column -= 1;
            } else {
                self.column += 1;
            }
        }
    }
}
