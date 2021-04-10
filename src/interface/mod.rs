use minifb::{Key, Window, WindowOptions};
use std::u32;

use crate::World;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::GUI_SCALE;
use crate::TICK_RATE_MS;

pub fn render(world: &mut World) {
    let mut buffer: Vec<u32> = vec![0; GRID_WIDTH * GRID_HEIGHT];

    let options = WindowOptions {
        scale: GUI_SCALE,
        ..WindowOptions::default()
    };

    let mut window = Window::new("Life", GRID_WIDTH, GRID_HEIGHT, options).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_millis(TICK_RATE_MS)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (pixel, cell) in buffer.iter_mut().zip(world.grid.iter()) {
            match cell.is_alive() {
                true => *pixel = 0x00_BBBBBB,
                false => *pixel = 0x00_222222,
            }
        }

        world.tick();

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT)
            .unwrap();
    }
}
