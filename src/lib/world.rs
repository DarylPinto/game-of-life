use crate::lib::Cell;
use crate::patterns::Pattern;
use crate::utils;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use ca_formats::rle::Rle;
use rand::prelude::*;
use std::error::Error;

#[derive(Debug)]
pub struct World {
    pub grid: Vec<Vec<Cell>>,
    pub time_stopped: bool,
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![Cell::default(); GRID_WIDTH]; GRID_HEIGHT],
            time_stopped: false,
        }
    }

    pub fn clear(&mut self) {
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                cell.die();
            }
        }
    }

    pub fn populate_randomly(&mut self, chance_of_life: f32) {
        let mut rng = thread_rng();
        let threshhold = (u8::MAX as f32 * chance_of_life) as u8;

        self.clear();

        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                let roll: u8 = rng.gen();
                if roll < threshhold {
                    cell.spawn();
                }
            }
        }
    }

    pub fn populate_from_pattern(&mut self, pattern: &Pattern) -> Result<(), Box<dyn Error>> {
        self.clear();

        let parsed_pattern = Rle::new(pattern.rle_string)?;

        let cells = parsed_pattern
            .map(|cell| cell.unwrap().position)
            .collect::<Vec<_>>();

        for c in cells.iter() {
            let col = c.0 as usize + ((GRID_WIDTH - pattern.width) / 2);
            let row = c.1 as usize + ((GRID_HEIGHT - pattern.height) / 2);
            self.grid[row][col].spawn();
        }

        Ok(())
    }

    pub fn tick(&mut self) {
        if self.time_stopped {
            return;
        }

        // First pass, update neighbor count for every cell
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                // If the cell is dead and has no neighors,
                // don't bother doing anything with it
                if self.grid[i][j].state == 0 {
                    continue;
                }

                // If the cell is alve, visit each neighbor and increment their
                // internal "living neighbor" state
                let is_alive = self.grid[i][j].is_alive();

                if is_alive {
                    let neighbor_positions =
                        utils::get_neighbor_positions(i, j, GRID_WIDTH, GRID_HEIGHT);

                    for pos in neighbor_positions.list.iter() {
                        match *pos {
                            Some((i, j)) => self.grid[i][j].increment_living_neighbor_count(),
                            None => (),
                        }
                    }
                }
            }
        }

        // Second pass, set living/dead state
        for i in 0..self.grid.len() {
            for j in 0..self.grid[i].len() {
                // If the cell is dead and has no neighors,
                // don't bother doing anything with it
                if self.grid[i][j].state == 0 {
                    continue;
                }

                // Any live cell with two or three live neighbours survives.
                // Any dead cell with three live neighbours becomes a live cell.
                // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
                let is_alive = self.grid[i][j].is_alive();
                let living_neighbors = self.grid[i][j].get_living_neighbor_count();

                if is_alive && (living_neighbors < 2 || living_neighbors > 3) {
                    self.grid[i][j].die();
                } else if !is_alive && living_neighbors == 3 {
                    self.grid[i][j].spawn();
                }

                // Set the living neighbor count back to 0
                self.grid[i][j].reset_neighbor_count();
            }
        }
    }
}
