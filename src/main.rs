#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lib;
mod patterns;
mod utils;

use lib::World;
use minifb::Scale;
use minifb::{Key, KeyRepeat, Menu, MouseButton, MouseMode, Window, WindowOptions};
use std::error::Error;

// Configuration
const WINDOW_TITLE: &str = "Game of Life";

const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = 200;

const CHANCE_OF_LIFE: f32 = 0.125;

const TICK_RATE_MS: u64 = 50;
const GUI_SCALE: Scale = Scale::X4;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new();
    let patterns = patterns::get_patterns();

    world.populate_randomly(CHANCE_OF_LIFE);

    // Main Window
    let mut window = Window::new(
        WINDOW_TITLE,
        GRID_WIDTH,
        GRID_HEIGHT,
        WindowOptions {
            scale: GUI_SCALE,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; GRID_WIDTH * GRID_HEIGHT];

    // Window Menus
    let mut menu = Menu::new("Patterns")?;

    for (i, pattern) in patterns.iter().enumerate() {
        menu.add_item(pattern.name, i + 1).build();
    }

    menu.add_item("Random", 0).build();

    window.add_menu(&menu);

    // Main loop
    window.limit_update_rate(Some(std::time::Duration::from_millis(TICK_RATE_MS)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw and advance forward in time
        utils::draw(&mut window, &mut buffer, &world)?;
        world.tick();

        // Menu handler
        window.is_menu_pressed().map(|menu_id| {
            world.time_stopped = false;

            if menu_id == 0 {
                world.populate_randomly(CHANCE_OF_LIFE);
                return;
            }

            let (_, pattern) = patterns
                .iter()
                .enumerate()
                .find(|(i, _)| *i + 1 == menu_id)
                .unwrap_or_else(|| (0, &patterns[0]));

            match world.populate_from_pattern(pattern) {
                Err(e) => panic!("Error loading from pattern: {:?}", e),
                _ => (),
            };
        });

        // Allow pausing time with spacebar
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            world.time_stopped = !world.time_stopped;
        }

        // Allow drawing with the mouse if time is paused
        if world.time_stopped {
            window.get_mouse_pos(MouseMode::Discard).map(|(x, y)| {
                if window.get_mouse_down(MouseButton::Left) {
                    world.grid[y as usize][x as usize].spawn();
                }
            });
        }
    }

    Ok(())
}
