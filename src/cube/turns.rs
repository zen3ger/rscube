use cube::cubie::Pos;
use std::iter::FromIterator;

pub trait Turnable {
    type Iter: Iterator<Item = Pos>;
    type FromIter: FromIterator<Pos>;

    fn iter_pos(&self) -> Self::Iter;

    fn has(&self, p: Pos) -> bool {
        self.iter_pos().any(|q| q == p)
    }

    fn map_pos<F>(&self, f: F) -> Option<Self::FromIter>
    where
        F: Fn(Pos) -> Option<Pos>,
    {
        self.iter_pos().map(f).collect::<Option<Self::FromIter>>()
    }

    // Face turns
    fn u(&self) -> Option<Self::FromIter> {
        if !self.has(Pos::U) {
            return None;
        }
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
        if !self.has(Pos::R) {
            return None;
        }
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
        if !self.has(Pos::F) {
            return None;
        }
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
        if !self.has(Pos::D) {
            return None;
        }
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
        if !self.has(Pos::L) {
            return None;
        }
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
        if !self.has(Pos::B) {
            return None;
        }
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
        self.map_pos(|p| match p {
            Pos::L | Pos::R => None,
            Pos::U => Some(Pos::F),
            Pos::D => Some(Pos::B),
            Pos::F => Some(Pos::D),
            Pos::B => Some(Pos::U),
        })
    }
    fn e(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::U | Pos::D => None,
            Pos::F => Some(Pos::R),
            Pos::L => Some(Pos::F),
            Pos::B => Some(Pos::L),
            Pos::R => Some(Pos::B),
        })
    }
    fn s(&self) -> Option<Self::FromIter> {
        self.map_pos(|p| match p {
            Pos::F | Pos::B => None,
            Pos::U => Some(Pos::R),
            Pos::D => Some(Pos::L),
            Pos::L => Some(Pos::U),
            Pos::R => Some(Pos::D),
        })
    }

    fn apply_turn(&self, mov: Turn) -> Option<Self::FromIter> {
        match mov {
            Turn::U => self.u(),
            Turn::R => self.r(),
            Turn::F => self.f(),
            Turn::D => self.d(),
            Turn::L => self.l(),
            Turn::B => self.b(),
            Turn::M => self.m(),
            Turn::E => self.e(),
            Turn::S => self.s(),
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

pub const TURNS: [Turn; 9] = [
    Turn::U,
    Turn::E,
    Turn::D,
    Turn::L,
    Turn::M,
    Turn::R,
    Turn::F,
    Turn::S,
    Turn::B,
];

impl Turn {
    pub fn is_slice(&self) -> bool {
        use self::Turn::*;

        match *self {
            M | E | S => true,
            _ => false,
        }
    }

    pub fn is_face(&self) -> bool {
        !self.is_slice()
    }
}
