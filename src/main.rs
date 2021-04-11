#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod interface;
mod lib;
mod utils;

use lib::World;
use minifb::Scale;
use rand::Rng;

// Configuration
const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 200;

const INITIAL_LIVING_CELL_COUNT: usize = 6000;

const TICK_RATE_MS: u64 = 50;
const GUI_SCALE: Scale = Scale::X4;

fn main() {
    let mut world = World::new();

    let mut initial_living_cell_coords = [0; INITIAL_LIVING_CELL_COUNT];

    let mut rng = rand::thread_rng();
    for coord in initial_living_cell_coords.iter_mut() {
        *coord = rng.gen_range(0..GRID_WIDTH * GRID_HEIGHT);
    }

    world.populate(&initial_living_cell_coords);

    interface::render(&mut world);
}
