use minifb::{Key, Window, WindowOptions};
use std::error::Error;

use crate::World;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use crate::GUI_SCALE;
use crate::TICK_RATE_MS;

pub fn render(world: &mut World) -> Result<(), Box<dyn Error>> {
    let mut buffer: Vec<u32> = vec![0; GRID_WIDTH * GRID_HEIGHT];

    let options = WindowOptions {
        scale: GUI_SCALE,
        ..WindowOptions::default()
    };

    let mut window = Window::new("Game of Life", GRID_WIDTH, GRID_HEIGHT, options)?;

    // window.limit_update_rate(Some(std::time::Duration::from_millis(TICK_RATE_MS)));
    window.limit_update_rate(None);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for y in 0..GRID_HEIGHT {
            for x in 0..GRID_WIDTH {
                buffer[(y * GRID_WIDTH) + x] = match world.grid[y][x].is_alive() {
                    true => u32::MAX - ((x * y) as u32),
                    false => 0x00_212121,
                }
            }
        }

        world.tick();

        window.update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT)?
    }

    Ok(())
}
