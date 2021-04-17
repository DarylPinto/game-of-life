use crate::lib::NeighborPositionList;
use crate::World;
use crate::GRID_HEIGHT;
use crate::GRID_WIDTH;
use minifb::Window;
use std::error::Error;

pub fn draw(
    window: &mut Window,
    buffer: &mut Vec<u32>,
    world: &World,
) -> Result<(), Box<dyn Error>> {
    for y in 0..GRID_HEIGHT {
        for x in 0..GRID_WIDTH {
            buffer[(y * GRID_WIDTH) + x] = match world.grid[y][x].is_alive() {
                true => 0x00_FFFFFF,
                false => 0x00_212121,
            }
        }
    }

    window.update_with_buffer(&buffer, GRID_WIDTH, GRID_HEIGHT)?;

    Ok(())
}

// Returns the indicies of *all* (living or dead) neighboring cells in the grid.
pub fn get_neighbor_positions(
    row_idx: usize,
    col_idx: usize,
    grid_width: usize,
    grid_height: usize,
) -> NeighborPositionList {
    let mut neighbors = NeighborPositionList::new();

    let is_left_edge = col_idx == 0;
    let is_right_edge = col_idx == grid_width - 1;
    let is_top_edge = row_idx == 0;
    let is_bottom_edge = row_idx == grid_height - 1;

    if !is_left_edge {
        neighbors.insert((row_idx, col_idx - 1));
    }
    if !is_right_edge {
        neighbors.insert((row_idx, col_idx + 1));
    }
    if !is_top_edge {
        neighbors.insert((row_idx - 1, col_idx));
    }
    if !is_bottom_edge {
        neighbors.insert((row_idx + 1, col_idx));
    }
    if !is_top_edge && !is_left_edge {
        neighbors.insert((row_idx - 1, col_idx - 1));
    }
    if !is_top_edge && !is_right_edge {
        neighbors.insert((row_idx - 1, col_idx + 1));
    }
    if !is_bottom_edge && !is_left_edge {
        neighbors.insert((row_idx + 1, col_idx - 1));
    }
    if !is_bottom_edge && !is_right_edge {
        neighbors.insert((row_idx + 1, col_idx + 1));
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbor_positions() {
        // Standard cell
        let neighbors = get_neighbor_positions(1, 1, 5, 4).debug_collapse();
        assert_eq!(
            neighbors,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2)
            ]
        );

        // Top left corner cell
        let neighbors = get_neighbor_positions(0, 0, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(0, 1), (1, 0), (1, 1)]);

        // Top right corner cell
        let neighbors = get_neighbor_positions(0, 4, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(0, 3), (1, 3), (1, 4)]);

        // Bottom left corner cell
        let neighbors = get_neighbor_positions(3, 0, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(2, 0), (2, 1), (3, 1)]);

        // Bottom right corner cell
        let neighbors = get_neighbor_positions(3, 4, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(2, 3), (2, 4), (3, 3)]);

        // Top edge cell
        let neighbors = get_neighbor_positions(0, 2, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(0, 1), (0, 3), (1, 1), (1, 2), (1, 3)]);

        // Bottom edge cell
        let neighbors = get_neighbor_positions(3, 3, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(2, 2), (2, 3), (2, 4), (3, 2), (3, 4)]);

        // Left edge cell
        let neighbors = get_neighbor_positions(2, 0, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(1, 0), (1, 1), (2, 1), (3, 0), (3, 1)]);

        // Right edge cell
        let neighbors = get_neighbor_positions(1, 4, 5, 4).debug_collapse();
        assert_eq!(neighbors, vec![(0, 3), (0, 4), (1, 3), (2, 3), (2, 4)]);
    }
}
