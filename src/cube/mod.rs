pub mod cubie;
pub mod turns;

use std;
use self::turns::Turnable;

#[derive(Debug)]
pub struct Cube {
    centers: [cubie::Center; 6],
    corners: [cubie::Corner; 8],
    edges: [cubie::Edge; 12],
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            centers: cubie::Center::centers(),
            corners: cubie::Corner::corners(),
            edges: cubie::Edge::edges(),
        }
    }

    pub fn turn(&mut self, turn: turns::Turn) {
        for corner in &mut self.corners {
            let newpos = (&corner.pos).apply_turn(turn).unwrap_or(None);
            if let Some(ps) = newpos {
                corner.pos = ps;
            }
        }

        for edge in &mut self.edges {
            let newpos = (&edge.pos).apply_turn(turn).unwrap_or(None);
            if let Some(ps) = newpos {
                edge.pos = ps;
            }
        }

        for center in &mut self.centers {
            let newpos = (&center.pos).apply_turn(turn).unwrap_or(None);
            if let Some(ps) = newpos {
                center.pos = ps;
            }
        }
    }

    pub fn list_any<F>(&self, f: F)
    where
        F: Fn(&cubie::Cubie) -> bool,
    {
        for corner in &self.corners {
            if f(corner) {
                println!("{:?}", corner)
            }
        }

        for edge in &self.edges {
            if f(edge) {
                println!("{:?}", edge)
            }
        }

        for center in &self.centers {
            if f(center) {
                println!("{:?}",center);
            }
        }
    }
}

impl std::default::Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}
