use alg;
use cube::Pos;
use std::iter::{FromIterator, IntoIterator};
use std::slice;

#[derive(Debug)]
pub struct Corner {
    init: CornerPos,
    pos: CornerPos,
}

impl Corner {
    fn new(ps: [Pos;3]) -> Self {
        let cp = CornerPos{ pos: ps };
        Self {
            init: cp,
            pos: cp,
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
            let p = *p;
            if !placed { break; }
            placed = self.init
                .into_iter()
                .fold(false, |acc, &i| acc || i == p);
        }
        placed
    }
}

impl alg::moves::Moveable for Corner {
    fn apply_move(&mut self, mov: alg::moves::Move) {
        self.pos.apply_move(mov);
    }
}


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct CornerPos {
    pos: [Pos; 3],
}

impl<'a> IntoIterator for &'a CornerPos {
    type Item = &'a Pos;
    type IntoIter = slice::Iter<'a, Pos>;

    fn into_iter(self) -> Self::IntoIter {
        self.pos.into_iter()
    }    
} 

impl FromIterator<Pos> for CornerPos {
    fn from_iter<I: IntoIterator<Item=Pos>>(iter: I) -> Self {
        let mut ps = [Pos::U;3];
        for (i,p) in iter.into_iter().enumerate() {
            if i == 3 { break; }
            ps[i] = p;
        }
        CornerPos { pos: ps }
    }
}

impl alg::moves::Moveable for CornerPos {
    /* Only face turns apply to corner cubies */

    fn u(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => Some(Pos::U),
                Pos::D => None,
                Pos::F => Some(Pos::L),
                Pos::L => Some(Pos::B),
                Pos::B => Some(Pos::R),
                Pos::R => Some(Pos::F),})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }

    fn r(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => Some(Pos::B),
                Pos::D => Some(Pos::F),
                Pos::F => Some(Pos::U),
                Pos::L => None,
                Pos::B => Some(Pos::D),
                Pos::R => Some(Pos::R),})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }

    fn f(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => Some(Pos::R),
                Pos::D => Some(Pos::L),
                Pos::F => Some(Pos::F),
                Pos::L => Some(Pos::U),
                Pos::B => None,
                Pos::R => Some(Pos::D),})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }

    fn d(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => None,
                Pos::D => Some(Pos::D),
                Pos::F => Some(Pos::R),
                Pos::L => Some(Pos::F),
                Pos::B => Some(Pos::L),
                Pos::R => Some(Pos::B),})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }

    fn l(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => Some(Pos::F),
                Pos::D => Some(Pos::B),
                Pos::F => Some(Pos::D),
                Pos::L => Some(Pos::L),
                Pos::B => Some(Pos::U),
                Pos::R => None,})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }

    fn b(&mut self) {
        let npos: Option<CornerPos> = self.pos
            .into_iter()
            .map(|p| match *p {
                Pos::U => Some(Pos::L),
                Pos::D => Some(Pos::R),
                Pos::F => None,
                Pos::L => Some(Pos::D),
                Pos::B => Some(Pos::B),
                Pos::R => Some(Pos::U),})
            .collect::<Option<CornerPos>>();
        match npos {
            Some(cp) => *self = cp,
            _ => {},
        }
    }
}
