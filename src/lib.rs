mod kalman;

use wasm_bindgen::prelude::*;
use std::fmt;
use kalman::Kalman;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Kalman = 1,
    Wall = 2
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Graph {
    width: u32,
    height: u32,
    kalman: Kalman,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Graph {
    /*
     * Static function for returning the 1D index from the 2D index.
     */
    pub fn get_index(width: u32, row: u32, column: u32) -> usize {
        (row * width + column) as usize
    }
    /*
     * Execute a single timestep.
     */
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();
        let mut old_index = Graph::get_index(self.width, self.kalman.row, self.kalman.column).clone();

        // Do something to the cells based on the decision of the agent.
        self.kalman.update_index(self.clone());
        let new_index = Graph::get_index(self.width, self.kalman.row, self.kalman.column).clone();

        // Update the old cell to empty and the new cell to Kalman
        next[old_index] = Cell::Empty;
        next[new_index] = Cell::Kalman;
        self.cells = next;
    }

    pub fn new(seed_w: u32, seed_h: u32) -> Graph {
        let width = 64;
        let height = 64;
        let kalman = Kalman::new(seed_w, seed_h);

        let cells = (0..width * height)
            .map(|i| {
                if i as usize == Graph::get_index(width, seed_w, seed_h).clone() {
                    Cell::Kalman
                } else {
                    Cell::Empty
                }
            })
            .collect();

        Graph {
            width,
            height,
            kalman,
            cells,
        }
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = match cell {
                    Cell::Empty => '◻',
                    Cell::Kalman => 'Κ',
                    Cell::Wall => '◼'
                };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, Kalman Demo!");
}