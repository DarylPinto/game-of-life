#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod lib;
mod patterns;
mod utils;

use lib::World;
use minifb::Scale;
use minifb::{Key, KeyRepeat, Menu, MouseButton, MouseMode, Window, WindowOptions};
use patterns::PATTERNS;
use std::{error::Error, time::Duration};

// Configuration
const WINDOW_TITLE: &str = "Game of Life";

const INITIAL_WORLD_WIDTH: usize = 300;
const INITIAL_WORLD_HEIGHT: usize = 200;

const CHANCE_OF_LIFE: f32 = 0.125;

const FRAMES_PER_SECOND: u64 = 40;
const UPDATE_RATE: Duration = Duration::from_millis(1000 / FRAMES_PER_SECOND);
const GUI_SCALE: Scale = Scale::X4;

fn main() -> Result<(), Box<dyn Error>> {
    let mut world = World::new(INITIAL_WORLD_WIDTH, INITIAL_WORLD_HEIGHT);

    world.populate_randomly(CHANCE_OF_LIFE);

    // Main Window
    let mut window = Window::new(
        WINDOW_TITLE,
        INITIAL_WORLD_WIDTH,
        INITIAL_WORLD_HEIGHT,
        WindowOptions {
            scale: GUI_SCALE,
            ..WindowOptions::default()
        },
    )?;

    let mut buffer: Vec<u32> = vec![0; INITIAL_WORLD_WIDTH * INITIAL_WORLD_HEIGHT];

    // Window Menus
    let mut menu = Menu::new("Patterns")?;

    for (i, pattern) in PATTERNS.iter().enumerate() {
        menu.add_item(pattern.name, i + 1).build();
    }

    menu.add_item("Random", 0).build();

    window.add_menu(&menu);

    // Main loop
    window.limit_update_rate(Some(UPDATE_RATE));

    // Previous mouse position of the mouse while left click was held down
    let mut prev_contiguous_mousedown_pos: Option<(isize, isize)> = None;
    let mut frame: u8 = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Draw every frame
        frame = frame.wrapping_add(1);
        utils::draw(&mut window, &mut buffer, &world)?;

        // Step forward in time every other frame
        if frame % 2 == 0 {
            world.tick();
        }

        // When a pattern is selected from the menu
        if let Some(menu_id) = window.is_menu_pressed() {
            // Pause the simulation
            world.time_stopped = false;

            // First menu item is the same as initial load (random cells)
            if menu_id == 0 {
                world.populate_randomly(CHANCE_OF_LIFE);
            } else {
                // Find the pattern selected by the user
                let (_, pattern) = PATTERNS
                    .iter()
                    .enumerate()
                    .find(|(i, _)| *i + 1 == menu_id)
                    .unwrap_or_else(|| (0, &PATTERNS[0]));

                // Resize the frame buffer to fit the new pattern
                buffer.resize(pattern.world_width * pattern.world_height, 0);

                // Create a new world & window with the width, height and scaling of the selected pattern
                window = Window::new(
                    WINDOW_TITLE,
                    pattern.world_width,
                    pattern.world_height,
                    WindowOptions {
                        scale: pattern.scale,
                        ..WindowOptions::default()
                    },
                )?;
                window.limit_update_rate(Some(UPDATE_RATE));
                world = World::new(pattern.world_width, pattern.world_height);

                // Populate the world from the pattern RLE & start the simulation
                world.populate_from_pattern(pattern)?;
            }
        };

        // Allow pausing time with spacebar
        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            world.time_stopped = !world.time_stopped;
        }

        // Allow drawing with the mouse if time is paused
        if world.time_stopped {
            if window.get_mouse_down(MouseButton::Left) {
                window.get_mouse_pos(MouseMode::Clamp).map(|(x, y)| {
                    // Spawning cells at the cursor position while the left
                    // mouse button is held results in large gaps in the
                    // drawing. This is because the mouse moves fast, but is
                    // only "polled" at the tickrate of the window. To resolve
                    // this, we draw line segments connecting each polled mouse
                    // position.
                    let current_mouse_pos = (x as isize, y as isize);
                    let cells_to_spawn =
                        utils::get_line_segment(prev_contiguous_mousedown_pos, current_mouse_pos);

                    for cell in cells_to_spawn {
                        world.grid[cell.1 as usize][cell.0 as usize].spawn();
                    }

                    prev_contiguous_mousedown_pos = Some(current_mouse_pos);
                });
            } else {
                prev_contiguous_mousedown_pos = None;
            }
        }
    }

    Ok(())
}
