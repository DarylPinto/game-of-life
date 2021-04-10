#[derive(Debug)]
pub struct NeighborIndexList {
    pub list: [Option<usize>; 8],
}

impl NeighborIndexList {
    pub fn new() -> Self {
        Self { list: [None; 8] }
    }

    pub fn insert(&mut self, to_insert: usize) {
        for (i, item) in self.list.iter_mut().enumerate() {
            match item {
                None => {
                    self.list[i] = Some(to_insert);
                    return;
                }
                _ => (),
            }
        }
        panic!("Cannot insert more than 8 values into NeighborIndexList");
    }

    // WARNING: This should ONLY be used for testing, as it's rather expensive to perform multiple times per frame
    // Collapse the sorted neighbor list into a sorted vector
    #[cfg(test)]
    pub fn debug_collapse(&self) -> Vec<usize> {
        let mut collapsed = self
            .list
            .iter()
            .filter_map(|item| *item)
            .collect::<Vec<usize>>();
        collapsed.sort();
        collapsed
    }
}
