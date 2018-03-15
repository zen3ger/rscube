pub mod cubie;
pub mod turns;

use self::turns::Turnable;
use self::cubie::{Center, Corner, Edge};

use rand;
use std;

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

    pub fn scramble(&mut self) {
        use rand::Rng;
        use self::turns::TURNS;

        let mut rng = rand::thread_rng();

        for _ in 0..rng.gen_range(23, 46) {
            let t = TURNS[rng.gen_range(0, 9)];
            self.turn(t)
        }
    }

    pub fn corners(&self) -> std::slice::Iter<Corner> {
        self.corners.iter()
    }

    pub fn edges(&self) -> std::slice::Iter<Edge> {
        self.edges.iter()
    }

    pub fn centers(&self) -> std::slice::Iter<Center> {
        self.centers.iter()
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
                println!("{:?}", center);
            }
        }
    }
}

impl std::default::Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}
