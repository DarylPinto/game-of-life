use rand::Rng;
use std::{convert::TryInto, thread, time};

// Configuration
const INITIAL_LIVING_CELL_COUNT: usize = 1500;
const TICK_RATE_MS: u64 = 100;

// World size
const GRID_WIDTH: usize = 50;
const GRID_AREA: usize = GRID_WIDTH.pow(2);

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

#[derive(Debug)]
struct World {
    prev_grid: [bool; GRID_AREA],
    grid: [bool; GRID_AREA],
}

impl World {
    fn new() -> Self {
        Self {
            prev_grid: [false; GRID_AREA],
            grid: [false; GRID_AREA],
        }
    }

    fn populate(&mut self, cell_coords: &[usize; INITIAL_LIVING_CELL_COUNT]) {
        for (i, cell) in self.grid.iter_mut().enumerate() {
            // If the current cell's index is in the `cell_coords` vec,
            // Make the cell alive
            if cell_coords.iter().any(|&coord| coord == i) {
                *cell = true;
            }
        }
    }

    fn draw(&self) {
        for chunk in self.grid.chunks(GRID_WIDTH) {
            let line: String = chunk
                .iter()
                .map(|cell| if *cell { "▇▇" } else { "  " })
                .collect();

            println!("{:?}", line);
            // println!("{}", line);
        }
    }

    fn living_neighbor_count(&self, cell_coord: isize) -> u8 {
        let width = GRID_WIDTH as isize;

        let mut neighbors = vec![
            // Top neighbors
            cell_coord - width - 1,
            cell_coord - width,
            cell_coord - width + 1,
            // Horizontal neighbors
            // cell_coord - 1,
            // cell_coord + 1,
            // Bottom neighbors
            cell_coord + width - 1,
            cell_coord + width,
            cell_coord + width + 1,
        ];

        if cell_coord % width != 0 {
            neighbors.push(cell_coord - 1);
        }

        if cell_coord % width != width - 1 {
            neighbors.push(cell_coord + 1);
        }

        neighbors.iter().fold(0, |acc, coord| {
            let coord = *coord;
            let length = (self.prev_grid.len() - 1) as isize;

            if coord < 0 || coord > length {
                acc
            } else if self.prev_grid[coord as usize] {
                acc + 1
            } else {
                acc
            }
        })
    }

    // Any live cell with two or three live neighbours survives.
    // Any dead cell with three live neighbours becomes a live cell.
    // All other live cells die in the next generation. Similarly, all other dead cells stay dead.
    fn tick(&mut self) {
        self.prev_grid = self.grid.clone();

        for i in 0..self.grid.len() {
            let living_neighbors = self.living_neighbor_count(i.try_into().unwrap());
            let cell = self.grid[i];

            if cell && (living_neighbors == 2 || living_neighbors == 3) {
                self.grid[i] = true;
            } else if !cell && living_neighbors == 3 {
                self.grid[i] = true;
            } else {
                self.grid[i] = false;
            }
        }
    }
}

fn main() {
    let mut world = World::new();

    // 180 :: 181 :: 182
    // 204 :: 205 :: 206
    // 228 :: 229 :: 230
    let mut initial_living_cell_coords = [0; INITIAL_LIVING_CELL_COUNT];

    let mut rng = rand::thread_rng();
    for i in 0..INITIAL_LIVING_CELL_COUNT {
        let coord = rng.gen_range(0..GRID_AREA);
        initial_living_cell_coords[i] = coord;
    }

    world.populate(&initial_living_cell_coords);

    loop {
        clear_screen();
        world.draw();
        world.tick();
        thread::sleep(time::Duration::from_millis(TICK_RATE_MS));
    }
}
