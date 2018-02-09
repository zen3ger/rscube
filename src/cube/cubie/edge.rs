use cube::cubie::{Cubie, Pos};
use cube::turns::Turnable;
use std::iter::{IntoIterator, FromIterator};

#[derive(Debug)]
pub struct Edge {
    init: EdgePos,
    pos: EdgePos,
}

impl Edge {
    fn new(ep: [Pos;2]) -> Edge {
        Self {
            init: EdgePos { pos: ep },
            pos: EdgePos { pos: ep },
        }
    }

    pub fn edges() -> [Edge; 12] {
        use cube::cubie::Pos::*;

        [ Edge::new([U,B])
        , Edge::new([U,R])
        , Edge::new([U,F])
        , Edge::new([U,L])
        , Edge::new([D,F])
        , Edge::new([D,R])
        , Edge::new([D,B])
        , Edge::new([D,L])
        , Edge::new([L,B])
        , Edge::new([R,B])
        , Edge::new([R,F])
        , Edge::new([L,F])
        ]
    }
}

impl Cubie for Edge {
    fn is_solved(&self) -> bool {
        self.init == self.pos
    }

    fn is_placed(&self) -> bool {
        let mut placed = true;
        for p in self.pos.into_iter() {
            if !placed { break; }
            placed = self.init
                .into_iter()
                .fold(false, |acc, i| acc || i == p);
        }
        placed
    }
}

#[derive(Debug, PartialEq, Eq)]
struct EdgePos {
    pos: [Pos;2],
}

struct EdgePosIter<'a> {
    edgepos: &'a EdgePos,
    index: usize,
}

impl<'a> Iterator for EdgePosIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.index {
            0|1 => self.edgepos.pos[self.index],
            _ => return None,
        };
        self.index += 1;
        Some(pos)
    }
}

impl<'a> IntoIterator for &'a EdgePos {
    type Item = Pos;
    type IntoIter = EdgePosIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        EdgePosIter {
            edgepos: self,
            index: 0,
        }
    }
}

impl FromIterator<Pos> for EdgePos {
    fn from_iter<I: IntoIterator<Item=Pos>>(iter: I) -> Self {
        let mut ps = [Pos::U; 2];

        for (i,p) in iter.into_iter().enumerate() {
            match i {
                0 => ps[i] = p,
                1 => {
                    ps[i] = p;
                    return EdgePos { pos: ps };
                },
                _ => break,
            }
        }
        panic!("EdgePos::from_iter() not enough item to create object!")
    }
}

impl<'a> Turnable for &'a EdgePos {
    type Iter = EdgePosIter<'a>;
    type FromIter = EdgePos;

    fn iter_pos(&self) -> Self::Iter { self.into_iter() }
}
