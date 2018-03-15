use cube::cubie::{Cubie, Pos};
use cube::turns::Turnable;
use std::iter::{FromIterator, IntoIterator};
use std::ops::Deref;

#[derive(Debug)]
pub struct Center {
    pub init: CenterPos,
    pub pos: CenterPos,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CenterPos {
    pub pos: Pos,
}

pub struct CenterPosIter<'a> {
    centerpos: &'a CenterPos,
    pulled: bool,
}

impl Center {
    fn new(p: Pos) -> Self {
        Self {
            init: CenterPos { pos: p },
            pos: CenterPos { pos: p },
        }
    }

    pub fn centers() -> [Self; 6] {
        use cube::cubie::Pos::*;

        [
            Self::new(U),
            Self::new(D),
            Self::new(R),
            Self::new(L),
            Self::new(F),
            Self::new(B),
        ]
    }

    pub fn pos(&self) -> Pos {
        self.pos.pos
    }
}

impl Cubie for Center {
    fn is_solved(&self) -> bool {
        self.init == self.pos
    }

    fn is_placed(&self) -> bool {
        self.is_solved()
    }

    fn id(&self) -> String {
        let mut id = String::with_capacity(1);
        id.push(self.pos.pos.as_char());
        id
    }
}

impl Deref for Center {
    type Target = CenterPos;

    fn deref(&self) -> &Self::Target {
        &self.pos
    }
}

impl CenterPos {
    fn new(p: Pos) -> Self {
        Self { pos: p }
    }
}

impl FromIterator<Pos> for Option<CenterPos> {
    fn from_iter<I: IntoIterator<Item = Pos>>(iter: I) -> Self {
        iter.into_iter().next().map(CenterPos::new)
    }
}

impl<'a> IntoIterator for &'a CenterPos {
    type Item = Pos;
    type IntoIter = CenterPosIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CenterPosIter {
            centerpos: self,
            pulled: false,
        }
    }
}

impl<'a> Turnable for &'a CenterPos {
    type Iter = CenterPosIter<'a>;
    type FromIter = Option<CenterPos>;

    fn iter_pos(&self) -> Self::Iter {
        self.into_iter()
    }

    fn u(&self) -> Option<Self::FromIter> {
        None
    }

    fn r(&self) -> Option<Self::FromIter> {
        None
    }

    fn f(&self) -> Option<Self::FromIter> {
        None
    }

    fn d(&self) -> Option<Self::FromIter> {
        None
    }

    fn l(&self) -> Option<Self::FromIter> {
        None
    }

    fn b(&self) -> Option<Self::FromIter> {
        None
    }
}

impl<'a> Iterator for CenterPosIter<'a> {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.pulled {
            self.pulled = true;
            Some(self.centerpos.pos)
        } else {
            None
        }
    }
}
