mod center;
mod corner;
mod edge;

pub use self::center::Center;
pub use self::corner::Corner;
pub use self::edge::Edge;

pub trait Cubie {
    fn is_placed(&self) -> bool;
    fn is_solved(&self) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pos {
    U,
    D,
    R,
    L,
    F,
    B,
}
