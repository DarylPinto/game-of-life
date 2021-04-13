#[derive(Debug)]
pub struct NeighborPositionList {
    pub list: [Option<(usize, usize)>; 8],
}

impl NeighborPositionList {
    pub fn new() -> Self {
        Self { list: [None; 8] }
    }

    pub fn insert(&mut self, to_insert: (usize, usize)) {
        for (i, item) in self.list.iter_mut().enumerate() {
            match item {
                None => {
                    self.list[i] = Some(to_insert);
                    return;
                }
                Some(_) => (),
            }
        }
        panic!("Cannot insert more than 8 values into NeighborPositionList");
    }

    // WARNING: This should ONLY be used for testing, as it's rather expensive to perform multiple times per frame
    // Collapse the sorted neighbor list into a sorted vector
    #[cfg(test)]
    pub fn debug_collapse(&self) -> Vec<(usize, usize)> {
        let mut collapsed = self
            .list
            .iter()
            .filter_map(|item| *item)
            .collect::<Vec<(usize, usize)>>();
        collapsed.sort();
        collapsed
    }
}
