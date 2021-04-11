use std::fmt;

// Cell "state" byte:
// [0][0][0][0] [0][0][0][0]
//                        |-- this bit determines if the cell is alive
//           |_________|---------- these 4 bits determine how many living neighbors the cell has (from 0 to 8)

//  The cell is the most fundemental structure in the game of life.
//  It's living state and neighbor count is contained in just 1 byte.
#[derive(Clone, Copy)]
pub struct Cell {
    pub state: u8,
}

impl Cell {
    pub fn new(initial_state: u8) -> Self {
        Cell {
            state: initial_state,
        }
    }

    // The least significant bit tells us if the cell is alive or dead
    pub fn is_alive(&self) -> bool {
        self.state & 1 == 1
    }

    // The rest of the bits count how many living neighbors the cell has.
    // We can easily read this by shifting the bits to the right by 1
    pub fn get_living_neighbor_count(&self) -> u8 {
        self.state >> 1
    }

    pub fn spawn(&mut self) {
        self.state = self.state | 1;
    }

    pub fn die(&mut self) {
        self.state = self.state & 0b11111110;
    }

    pub fn increment_living_neighbor_count(&mut self) {
        let count = self.get_living_neighbor_count();
        let lsb = self.state & 1;
        self.state = ((count + 1) << 1) + lsb;
    }

    // Set neighbor count to 0
    pub fn reset_neighbor_count(&mut self) {
        self.state = self.state & 1;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(0)
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Cell")
            .field("Is alive", &self.is_alive())
            .field("Living neighbor count", &self.get_living_neighbor_count())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell() {
        let mut cell = Cell::default();
        assert_eq!(cell.state, 0);

        cell.spawn();
        assert_eq!(cell.state, 1);

        for _ in 0..3 {
            cell.increment_living_neighbor_count();
        }
        assert_eq!(cell.state, 0b00000111);

        cell.die();
        assert_eq!(cell.state, 0b00000110);

        for _ in 0..5 {
            cell.increment_living_neighbor_count();
        }
        assert_eq!(cell.state, 0b00010000);

        cell.spawn();
        assert_eq!(cell.state, 0b00010001);

        assert_eq!(cell.is_alive(), true);
        assert_eq!(cell.get_living_neighbor_count(), 8);

        cell.reset_neighbor_count();
        assert_eq!(cell.state, 1);
    }
}
