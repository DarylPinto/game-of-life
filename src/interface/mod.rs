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

    window.limit_update_rate(Some(std::time::Duration::from_millis(TICK_RATE_MS)));

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (i, (pixel, cell)) in buffer.iter_mut().zip(world.grid.iter()).enumerate() {
            *pixel = match cell.is_alive() {
                true => u32::MAX - (i as u32),
                false => 0x00_212121,
            }
        }

        world.tick();

        window.update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT)?
    }

    Ok(())
}
