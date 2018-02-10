pub mod cubie;
pub mod turns;

use std::default::Default;

#[derive(Debug)]
pub struct Cube {
    corners: [cubie::Corner; 8],
    edges: [cubie::Edge; 12],
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            corners: cubie::Corner::corners(),
            edges: cubie::Edge::edges(),
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}
