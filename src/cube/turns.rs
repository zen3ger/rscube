use cube::cubie::Pos;
use std::iter::FromIterator;

pub trait Turnable {
    type Iter: Iterator<Item = Pos>;
    type FromIter: FromIterator<Pos>;

    fn iter_pos(&self) -> Self::Iter;

    fn map_pos<F>(&self, f: F) -> Option<Self::FromIter>
    where
        F: Fn(Pos) -> Option<Pos>,
    {
        self.iter_pos()
            .map(|p| f(p))
            .collect::<Option<Self::FromIter>>()
    }

    // Face turns
    fn u(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::U),
            Pos::D => None,
            Pos::F => Some(Pos::L),
            Pos::L => Some(Pos::B),
            Pos::B => Some(Pos::R),
            Pos::R => Some(Pos::F),
        })
    }

    fn r(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::B),
            Pos::D => Some(Pos::F),
            Pos::F => Some(Pos::U),
            Pos::L => None,
            Pos::B => Some(Pos::D),
            Pos::R => Some(Pos::R),
        })
    }

    fn f(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::R),
            Pos::D => Some(Pos::L),
            Pos::F => Some(Pos::F),
            Pos::L => Some(Pos::U),
            Pos::B => None,
            Pos::R => Some(Pos::D),
        })
    }

    fn d(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => None,
            Pos::D => Some(Pos::D),
            Pos::F => Some(Pos::R),
            Pos::L => Some(Pos::F),
            Pos::B => Some(Pos::L),
            Pos::R => Some(Pos::B),
        })
    }

    fn l(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::F),
            Pos::D => Some(Pos::B),
            Pos::F => Some(Pos::D),
            Pos::L => Some(Pos::L),
            Pos::B => Some(Pos::U),
            Pos::R => None,
        })
    }

    fn b(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::L),
            Pos::D => Some(Pos::R),
            Pos::F => None,
            Pos::L => Some(Pos::D),
            Pos::B => Some(Pos::B),
            Pos::R => Some(Pos::U),
        })
    }
    // Slice turns
    fn m(&self) -> Option<Self::FromIter> {
        unimplemented!()
    }
    fn e(&self) -> Option<Self::FromIter> {
        unimplemented!()
    }
    fn s(&self) -> Option<Self::FromIter> {
        unimplemented!()
    }

    fn apply_move(&self, mov: Turn) -> Option<Self::FromIter> {
        use self::Turn::*;

        match mov {
            U => self.u(),
            R => self.r(),
            F => self.f(),
            D => self.d(),
            L => self.l(),
            B => self.b(),
            M => self.m(),
            E => self.e(),
            S => self.s(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Turn {
    U,
    R,
    F,
    D,
    L,
    B,
    M,
    E,
    S,
}
