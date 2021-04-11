use crate::lib::Cell;
use crate::utils;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use rand::prelude::*;

#[derive(Debug)]
pub struct World {
    pub grid: Vec<Cell>,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: vec![Cell::default(); GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn populate_randomly(&mut self, chance_of_life: f32) {
        let mut rng = thread_rng();
        let threshhold = (u8::MAX as f32 * chance_of_life) as u8;

        for cell in self.grid.iter_mut() {
            let roll: u8 = rng.gen();
            if roll < threshhold {
                cell.spawn();
            }
        }
    }

    pub fn tick(&mut self) {
        // First pass, update neighbor count for every cell
        for i in 0..self.grid.len() {
            // If the cell is dead and has no neighors,
            // don't bother doing anything with it
            if self.grid[i].state == 0 {
                continue;
            }

            // If the cell is alve, visit each neighbor and increment their
            // internal "living neighbor" state
            let is_alive = self.grid[i].is_alive();

            if is_alive {
                let neighbor_indicies = utils::get_neighbor_indicies(i, GRID_WIDTH, GRID_HEIGHT);

                for neighbor_index in neighbor_indicies.list.iter() {
                    match *neighbor_index {
                        Some(index) => self.grid[index].increment_living_neighbor_count(),
                        None => (),
                    }
                }
            }
        }

        // Second pass, set living/dead state
        for i in 0..self.grid.len() {
            // If the cell is dead and has no neighors,
            // don't bother doing anything with it
            if self.grid[i].state == 0 {
                continue;
            }

            // Any live cell with two or three live neighbours survives.
            // Any dead cell with three live neighbours becomes a live cell.
            // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
            let is_alive = self.grid[i].is_alive();
            let living_neighbors = self.grid[i].get_living_neighbor_count();

            if is_alive && (living_neighbors < 2 || living_neighbors > 3) {
                self.grid[i].die();
            } else if !is_alive && living_neighbors == 3 {
                self.grid[i].spawn();
            }

            // Set the living neighbor count back to 0
            self.grid[i].reset_neighbor_count();
        }
    }
}
