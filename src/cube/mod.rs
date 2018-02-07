pub mod cubie;

use alg;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pos {
    U,
    D,
    R,
    L,
    F,
    B,
}

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

//impl alg::moves::Moveable<Pos> for Cube {
//   fn apply_move(&mut self, mov: alg::moves::Move) {
//        for corner in self.corners.iter_mut() {
//            corner.apply_move(mov);
//        }
//   } 
//}
