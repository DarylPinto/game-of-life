use crate::lib::Cell;
use crate::utils;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::INITIAL_LIVING_CELL_COUNT;

#[derive(Debug)]
pub struct World {
    pub grid: [Cell; GRID_WIDTH * GRID_HEIGHT],
}

impl World {
    pub fn new() -> Self {
        Self {
            grid: [Cell::default(); GRID_WIDTH * GRID_HEIGHT],
        }
    }

    pub fn populate(&mut self, cell_coords: &[usize; INITIAL_LIVING_CELL_COUNT]) {
        for coord in cell_coords {
            self.grid[*coord].spawn();
        }
    }

    // Any live cell with two or three live neighbours survives.
    // Any dead cell with three live neighbours becomes a live cell.
    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    pub fn tick(&mut self) {
        for i in 0..self.grid.len() {
            let cell = self.grid[i];
            let is_alive = cell.is_alive();

            // Update neighbor count for each cell
            if cell.state > 0 {
                let neighbor_indicies = utils::get_neighbor_indicies(i, GRID_WIDTH, GRID_HEIGHT);

                if is_alive {
                    for neighbor_index in neighbor_indicies.list.iter() {
                        match *neighbor_index {
                            Some(index) => self.grid[index].increment_living_neighbor_count(),
                            None => (),
                        }
                    }
                }
            }
        }

        for i in 0..self.grid.len() {
            let cell = self.grid[i];
            let is_alive = cell.is_alive();
            let living_neighbors = cell.get_living_neighbor_count();

            // Update living state based on neighbors
            if is_alive && (living_neighbors == 2 || living_neighbors == 3) {
                self.grid[i].spawn();
            } else if !is_alive && living_neighbors == 3 {
                self.grid[i].spawn();
            } else {
                self.grid[i].die();
            }

            // Set the neighbor count back to 0 for each cell
            self.grid[i].reset_neighbor_count();
        }
    }
}
