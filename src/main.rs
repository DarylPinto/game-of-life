#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod interface;
mod lib;
mod utils;

use lib::World;
use minifb::Scale;
use std::error::Error;

// Configuration
const GRID_WIDTH: usize = 1200;
const GRID_HEIGHT: usize = 1200;

// Range: 0 - 1
const CHANCE_OF_LIFE: f32 = 0.125;

const TICK_RATE_MS: u64 = 33;
const GUI_SCALE: Scale = Scale::X1;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();

    world.populate_from_pattern()?;

    // world.populate_randomly(CHANCE_OF_LIFE);

    interface::render(&mut world)?;

    Ok(())
}
