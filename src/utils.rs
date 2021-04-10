use crate::lib::NeighborIndexList;

// Returns the indicies of *all* (living or dead) neighboring cells in the grid.
pub fn get_neighbor_indicies(
    index: usize,
    grid_width: usize,
    grid_height: usize,
) -> NeighborIndexList {
    let mut neighbors = NeighborIndexList::new();

    let grid_area = grid_width * grid_height;

    let is_left_edge = index % grid_width == 0;
    let is_right_edge = index % grid_width == grid_width - 1;
    let is_top_edge = index < grid_width;
    let is_bottom_edge = index >= grid_area - grid_width;

    if !is_left_edge {
        neighbors.insert(index - 1);
    }
    if !is_right_edge {
        neighbors.insert(index + 1);
    }
    if !is_top_edge {
        neighbors.insert(index - grid_width);
    }
    if !is_bottom_edge {
        neighbors.insert(index + grid_width);
    }
    if !is_top_edge && !is_left_edge {
        neighbors.insert(index - grid_width - 1);
    }
    if !is_top_edge && !is_right_edge {
        neighbors.insert(index - grid_width + 1);
    }
    if !is_bottom_edge && !is_left_edge {
        neighbors.insert(index + grid_width - 1);
    }
    if !is_bottom_edge && !is_right_edge {
        neighbors.insert(index + grid_width + 1);
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_neighbor_indicies() {
        // Standard cell
        let neighbors = get_neighbor_indicies(5, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![0, 1, 2, 4, 6, 8, 9, 10]);

        // Top left corner cell
        let neighbors = get_neighbor_indicies(0, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![1, 4, 5]);

        // Top right corner cell
        let neighbors = get_neighbor_indicies(3, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![2, 6, 7]);

        // Bottom left corner cell
        let neighbors = get_neighbor_indicies(12, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![8, 9, 13]);

        // Bottom right corner cell
        let neighbors = get_neighbor_indicies(15, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![10, 11, 14]);

        // Top edge cell
        let neighbors = get_neighbor_indicies(2, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![1, 3, 5, 6, 7]);

        // Bottom edge cell
        let neighbors = get_neighbor_indicies(13, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![8, 9, 10, 12, 14]);

        // Left edge cell
        let neighbors = get_neighbor_indicies(8, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![4, 5, 9, 12, 13]);

        // Right edge cell
        let neighbors = get_neighbor_indicies(7, 4, 4).debug_collapse();
        assert_eq!(neighbors, vec![2, 3, 6, 10, 11]);
    }
}
