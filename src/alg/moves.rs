pub trait Moveable {
    // Face turns
    fn u(&mut self) {}
    fn r(&mut self) {}
    fn f(&mut self) {}
    fn d(&mut self) {}
    fn l(&mut self) {}
    fn b(&mut self) {}

    // Slice turns
    fn m(&mut self) {}
    fn e(&mut self) {}
    fn s(&mut self) {}

    fn apply_move(&mut self, mov: Move) {
        use alg::moves::Move::*;

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
