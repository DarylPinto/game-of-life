// Returns the indicies of the neighboring cells in the grid.
// Yes this function is hella ugly.
pub fn get_neighbor_indicies(index: usize, grid_width: usize, grid_height: usize) -> Vec<usize> {
    let first_index = 0;
    let last_index = grid_width * grid_height - 1;
    let mut neighbors = vec![];

    // Neighbors to the top
    if index > grid_width {
        neighbors.push(index - grid_width);
        if index > first_index {
            neighbors.push(index - grid_width - 1);
        }
        if index < last_index {
            neighbors.push(index - grid_width + 1);
        }
    }

    // Neighbors to the bottom
    if index < last_index - grid_width {
        neighbors.push(index + grid_width);
        if index < last_index {
            neighbors.push(index + grid_width + 1);
        }
        if index > first_index {
            neighbors.push(index + grid_width - 1);
        }
    }

    // Neighbors to the sides
    if index % grid_width != 0 {
        neighbors.push(index - 1);
    }

    if index % grid_width != grid_width - 1 {
        neighbors.push(index + 1);
    }

    neighbors
}
