use cube::Pos;
use std::iter::FromIterator;

pub trait Moveable {
    type PosIter: Iterator<Item=Pos>;
    type FromPos: FromIterator<Pos>;

    // This should be the only thing needed...
    fn pos(&self) -> Self::PosIter;
    fn new_pos(&mut self, Self::FromPos);

    fn map_pos<F>(&mut self, f: F)
        where F: Fn(Pos) -> Option<Pos>
    {
        let newpos = self.pos()
        .map(|p| { f(p) })
        .collect::<Option<Self::FromPos>>();

        match newpos {
            Some(pos) => self.new_pos(pos),
            None => {},
        }
    }

    // Face turns
    fn u(&mut self) {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::U),
            Pos::D => None,
            Pos::F => Some(Pos::L),
            Pos::L => Some(Pos::B),
            Pos::B => Some(Pos::R),
            Pos::R => Some(Pos::F),
        })
    }

    fn r(&mut self) {
       self.map_pos(|p| match p {
           Pos::U => Some(Pos::B),
           Pos::D => Some(Pos::F),
           Pos::F => Some(Pos::U),
           Pos::L => None,
           Pos::B => Some(Pos::D),
           Pos::R => Some(Pos::R),})
    }

    fn f(&mut self) {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::R),
            Pos::D => Some(Pos::L),
            Pos::F => Some(Pos::F),
            Pos::L => Some(Pos::U),
            Pos::B => None,
            Pos::R => Some(Pos::D),
        })
    }

    fn d(&mut self) {
       self.map_pos(|p| match p {
           Pos::U => None,
           Pos::D => Some(Pos::D),
           Pos::F => Some(Pos::R),
           Pos::L => Some(Pos::F),
           Pos::B => Some(Pos::L),
           Pos::R => Some(Pos::B),
       })
    }

    fn l(&mut self) {
        self.map_pos(|p| match p {
            Pos::U => Some(Pos::F),
            Pos::D => Some(Pos::B),
            Pos::F => Some(Pos::D),
            Pos::L => Some(Pos::L),
            Pos::B => Some(Pos::U),
            Pos::R => None,
        })
    }

    fn b(&mut self) {
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
    fn m(&mut self) { panic!() }
    fn e(&mut self) { panic!() }
    fn s(&mut self) { panic!() }

    fn apply_move(&mut self, mov: Move) {
        use self::Move::*;

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

// Wide moves will be a combination of two,
// prime moves will be 3turns instead of 1
// for sake of simplicity.

#[derive(Debug, Copy, Clone)]
pub enum Move {
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
