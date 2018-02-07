use alg;
use cube::Pos;
use std::iter::{IntoIterator, FromIterator};
use std::slice;

#[derive(Debug)]
pub struct Edge {
    init: EdgePos,
    pos: EdgePos,
}

impl Edge {
    fn new(ep: [Pos;2]) -> Edge {
        Edge {
            init: EdgePos { pos: ep },
            pos: EdgePos { pos: ep },
        }
    }

    pub fn edges() -> [Edge; 12] {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct EdgePos {
    pos: [Pos;2],
}
