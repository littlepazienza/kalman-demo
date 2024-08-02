use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Empty = 0,
    Kalman = 1,
    Wall = 2
}

pub struct Kalman {
    row: u32,
    column: u32
}

#[wasm_bindgen]
pub struct Graph {
    width: u32,
    height: u32,
    kalman: Kalman,
    cells: Vec<Cell>,
}

impl Graph {

    /*
     * Convert the 2D ref to a cell to its real vector index.
     */
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    // ...
}

#[wasm_bindgen]
impl Graph {
    /*
     * Execute a single timestep.
     */
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        // Do something to the cells based on the decision of the agent.


        self.cells = next;
    }

    pub fn new(seed_w: u32, seed_h: u32) -> Graph {
        let width = 64;
        let height = 64;
        let kalman = Kalman {
            row: seed_w,
            column: seed_h
        };

        let cells = (0..width * height)
            .map(|i| {
                if i == seed_w * width + seed_h {
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