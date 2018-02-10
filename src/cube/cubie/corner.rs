use cube::cubie::{Cubie, Pos};
use cube::turns::Turnable;
use std::iter::{FromIterator, IntoIterator};

#[derive(Debug)]
pub struct Corner {
    init: CornerPos,
    pos: CornerPos,
}

#[derive(Debug, PartialEq, Eq)]
struct CornerPos {
    pos: [Pos; 3],
}

struct CornerPosIter<'a> {
    cornerpos: &'a CornerPos,
    index: usize,
}

impl Corner {
    fn new(ps: [Pos; 3]) -> Self {
        Self {
            init: CornerPos { pos: ps },
            pos: CornerPos { pos: ps },
        }
    }

    pub fn corners() -> [Self; 8] {
        use cube::cubie::Pos::*;

        [
            Corner::new([U, L, B]),
            Corner::new([U, R, B]),
            Corner::new([U, R, F]),
            Corner::new([U, L, F]),
            Corner::new([D, L, F]),
            Corner::new([D, R, F]),
            Corner::new([D, R, B]),
            Corner::new([D, L, B]),
        ]
    }
}

impl Cubie for Corner {
    fn is_solved(&self) -> bool {
        self.init == self.pos
    }

    fn is_placed(&self) -> bool {
        let mut placed = true;
        for p in &self.pos {
            if !placed {
                break;
            }
            placed = self.init.into_iter().any(|i| i == p);
        }
        placed
    }
}

impl FromIterator<Pos> for CornerPos {
    fn from_iter<I: IntoIterator<Item = Pos>>(iter: I) -> Self {
        let mut ps = [Pos::U; 3];

        for (i, p) in iter.into_iter().enumerate() {
            match i {
                0 | 1 => ps[i] = p,
                2 => {
                    ps[i] = p;
                    return CornerPos { pos: ps };
                }
                _ => break,
            }
        }
        panic!("CornerPos::from_iter() not enough item to create object!");
    }
}

impl<'a> IntoIterator for &'a CornerPos {
    type Item = Pos;
    type IntoIter = CornerPosIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CornerPosIter {
            cornerpos: self,
            index: 0,
        }
    }
}

impl<'a> Turnable for &'a CornerPos {
    type Iter = CornerPosIter<'a>;
    type FromIter = CornerPos;

    fn iter_pos(&self) -> Self::Iter {
        self.into_iter()
    }

    // Slice moves don't apply to corner cubies
    fn m(&self) -> Option<Self::FromIter> {
        None
    }
    fn e(&self) -> Option<Self::FromIter> {
        None
    }
    fn s(&self) -> Option<Self::FromIter> {
        None
    }
}

impl<'a> Iterator for CornerPosIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.index {
            0 | 1 | 2 => self.cornerpos.pos[self.index],
            _ => return None,
        };
        self.index += 1;
        Some(pos)
    }
}
