use std::str::FromStr;
use cube::turns::Turn;

#[derive(Debug, Clone)]
pub enum MoveType {
    Single(Turn),
    Double(Turn),
    Prime(Turn),
}

#[derive(Debug)]
pub enum Move {
    Normal(MoveType),
    Wide(MoveType, MoveType),
}

impl MoveType {
    fn get(&self) -> Turn {
        use self::MoveType::*;

        match *self {
            Single(t) | Prime(t) | Double(t) => t,
        }
    }

    fn unwrap(self) -> Turn {
        self.get()
    }

    fn apply_double(&self) -> Self {
        use self::MoveType::*;

        match *self {
            Prime(t) | Single(t) => Double(t),
            Double(t) => Single(t),
        }
    }

    fn apply_prime(&self) -> Self {
        use self::MoveType::*;

        match *self {
            Prime(t) => Single(t),
            Single(t) => Prime(t),
            Double(t) => Double(t),
        }
    }

    pub fn wide_pair(&self) -> Self {
        use cube::turns::Turn::*;
        use self::MoveType::*;

        let p = match self.get() {
            U => Prime(E),
            D => Single(E),
            R => Prime(M),
            L => Single(M),
            F => Single(S),
            B => Prime(S),
            M | E | S => panic!("Slice `Turn`s don't have wide variants!"),
        };

        match *self {
            Single(_) => p,
            Prime(_) => p.apply_prime(),
            Double(_) => p.apply_double(),
        }
    }
}

impl Move {
    fn wide(&mut self) {
        use self::Move::*;

        let m = match *self {
            Normal(ref tt) => if !tt.get().is_slice() {
                Wide(tt.clone(), tt.wide_pair())
            } else {
                Normal(tt.clone())
            },
            Wide(ref tt, ref ttp) => Wide(tt.clone(), ttp.clone()),
        };
        *self = m;
    }

    fn apply<F>(&mut self, f: F)
    where
        F: Fn(&MoveType) -> MoveType,
    {
        use self::Move::*;

        *self = match *self {
            Normal(ref tt) => Normal(f(tt)),
            Wide(ref tt, ref ttp) => Wide(f(tt), f(ttp)),
        };
    }
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use cube::turns::Turn::*;
        use self::Move::*;
        use self::MoveType::*;

        if s.is_empty() {
            return Err("Failed to parse `Move`: empty input!");
        }

        let mut m = Normal(Single(U));

        for (i, ch) in s.chars().enumerate() {
            match i {
                0 => {
                    m = match ch {
                        'U' | 'u' => m,
                        'D' | 'd' => Normal(Single(D)),
                        'R' | 'r' => Normal(Single(R)),
                        'L' | 'l' => Normal(Single(L)),
                        'F' | 'f' => Normal(Single(F)),
                        'B' | 'b' => Normal(Single(B)),
                        'M' | 'm' => Normal(Single(M)),
                        'E' | 'e' => Normal(Single(E)),
                        'S' | 's' => Normal(Single(S)),
                        _ => return Err("Failed to parse `Move`: unknown `MoveType` type!"),
                    }
                }
                _ => match ch {
                    'W' | 'w' => m.wide(),
                    '\'' | 'i' => m.apply(MoveType::apply_prime),
                    '2' => m.apply(MoveType::apply_double),
                    _ => return Err("Failed to parse `Move`: unknown modifier!"),
                },
            }
        }
        Ok(m)
    }
}
