use alg;
use cube::Pos;
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::boxed::Box;


#[derive(Debug)]
pub struct Corner {
    init: CornerPos,
    pos: CornerPos,
}

impl Corner {
    fn new(ps: [Pos;3]) -> Self {
        let cp = CornerPos{ pos: ps };
        Self {
            init: cp.clone(),
            pos: cp.clone(),
        }
    }

    pub fn corners() -> [Self; 8] {
        use cube::Pos::*;

        [ Corner::new([U,L,B])
        , Corner::new([U,R,B])
        , Corner::new([U,R,F])
        , Corner::new([U,L,F])
        , Corner::new([D,L,F])
        , Corner::new([D,R,F])
        , Corner::new([D,R,B])
        , Corner::new([D,L,B])
        ]
    }

    pub fn is_solved(&self) -> bool { self.init == self.pos }

    pub fn is_placed(&self) -> bool {
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

//#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[derive(Clone, Debug, PartialEq, Eq)]
struct CornerPos {
    pos: [Pos; 3],
}

struct PosIter {
    cornerpos: CornerPos,
    index: usize,
}

impl Iterator for PosIter {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.index {
            0|1|2 => self.cornerpos.pos[self.index],
            _ => return None,
        };
        self.index += 1;
        Some(pos)
    }
}


impl<'a> IntoIterator for &'a CornerPos {
    type Item = Pos;
    type IntoIter = PosIter;

    fn into_iter(self) -> Self::IntoIter {
        PosIter {
            cornerpos: self.clone() ,
            index: 0,
        }
    } 
} 

impl FromIterator<Pos> for CornerPos {
    fn from_iter<I: IntoIterator<Item=Pos>>(iter: I) -> Self {
        let mut ps =[Pos::U; 3];

        for (i,p) in iter.into_iter().enumerate() {
            match i {
                0|1 => ps[i] = p,
                2 => {
                    ps[i] = p;
                    return CornerPos { pos: ps };
                },
                _ => {},
            }
        }
        panic!("CornerPos::from_iter() not enough item to create object!");
    }
}

impl alg::moves::Moveable for CornerPos {
    type PosIter = PosIter;
    type FromPos = Self;

    fn pos(&self) -> Self::PosIter { self.into_iter() }
    fn new_pos(&mut self, pos: Self::FromPos) { self.pos = pos.pos; }

    // Slice moves don't apply to corner cubies
    fn m(&mut self) {}
    fn e(&mut self) {}
    fn s(&mut self) {}
}
