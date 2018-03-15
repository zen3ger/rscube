use cube::cubie::{Cubie, Pos};
use cube::turns::Turnable;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Deref;

#[derive(Debug)]
pub struct Edge {
    pub init: EdgePos,
    pub pos: EdgePos,
}

#[derive(Debug, PartialEq, Eq)]
pub struct EdgePos {
    pub pos: [Pos; 2],
}

pub struct EdgePosIter<'a> {
    edgepos: &'a EdgePos,
    index: usize,
}

impl Edge {
    fn new(ep: [Pos; 2]) -> Self {
        Self {
            init: EdgePos { pos: ep },
            pos: EdgePos { pos: ep },
        }
    }

    pub fn edges() -> [Self; 12] {
        use cube::cubie::Pos::*;

        [
            Self::new([U, B]),
            Self::new([U, R]),
            Self::new([U, F]),
            Self::new([U, L]),
            Self::new([D, F]),
            Self::new([D, R]),
            Self::new([D, B]),
            Self::new([D, L]),
            Self::new([L, B]),
            Self::new([R, B]),
            Self::new([R, F]),
            Self::new([L, F]),
        ]
    }
}

impl Cubie for Edge {
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

    fn id(&self) -> String {
        let mut id: [char; 2] = ['-'; 2];
        for p in &self.pos {
            match p {
                Pos::U | Pos::D => {
                    if id[0] != '-' {
                        id[1] = id[0];
                    }
                    id[0] = p.as_char();
                }
                Pos::R | Pos::L => {
                    let mut i = 1;
                    if id[0] == '-' {
                        i = 0;
                    }
                    id[i] = p.as_char();
                }
                Pos::F | Pos::B => id[1] = p.as_char(),
            }
        }
        id.into_iter().collect::<String>()
    }
}

impl Deref for Edge {
    type Target = EdgePos;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl FromIterator<Pos> for Option<EdgePos> {
    fn from_iter<I: IntoIterator<Item = Pos>>(iter: I) -> Self {
        let mut ps = [Pos::U; 2];

        for (i, p) in iter.into_iter().enumerate() {
            match i {
                0 => ps[i] = p,
                1 => {
                    ps[i] = p;
                    return Some(EdgePos { pos: ps });
                }
                _ => break,
            }
        }
        None
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

impl<'a> Turnable for &'a EdgePos {
    type Iter = EdgePosIter<'a>;
    type FromIter = Option<EdgePos>;

    fn iter_pos(&self) -> Self::Iter {
        self.into_iter()
    }
}

impl<'a> Iterator for EdgePosIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.index {
            0 | 1 => self.edgepos.pos[self.index],
            _ => return None,
        };
        self.index += 1;
        Some(pos)
    }
}
