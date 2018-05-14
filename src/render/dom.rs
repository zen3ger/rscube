use resvg::{self, tree};
use resvg::tree::NodeExt;
use std::default;

use cube::Cube;
use cube::cubie::{Cubie, Pos};

pub struct Dom {
    pub rtree: resvg::tree::Tree,
    colors: ColorScheme,
}

pub struct ColorScheme {
    u: tree::Color,
    d: tree::Color,
    r: tree::Color,
    l: tree::Color,
    f: tree::Color,
    b: tree::Color,
}

impl Dom {
    pub fn load(path: &str) -> Self {
        let opt = resvg::Options::default();
        let rtree = resvg::parse_rtree_from_file(path, &opt).unwrap();

        Dom {
            rtree: rtree,
            colors: ColorScheme::default(),
        }
    }

    pub fn update(&mut self, cube: &Cube) {
       for mut node in self.rtree.root().descendants() {
            let id = String::from(&*node.id());

            if let tree::NodeKind::Path(ref mut p) = **node.borrow_mut() {
                match id.len() {
                    5 => {
                        // corners format PPP-F
                        let mut split = id.split('-');
                        // This two should always be there
                        // otherwise the svg format of the cube is invalid
                        let piece = split.next().unwrap();
                        let face = split.next().unwrap();
                        for corner in &mut cube.corners() {
                            if corner.id() == piece {
                                let (i, _) = corner
                                    .pos
                                    .into_iter()
                                    .map(|p| p.as_char())
                                    .enumerate()
                                    .find(|&(_, c)| Some(c) == face.chars().nth(0))
                                    .unwrap();
                                let c = Dom::match_color(&self.colors, &corner.init.pos[i]);
                                if let Some(ref mut fill) = p.fill {
                                    fill.paint = tree::Paint::Color(c);
                                }
                                break;
                            }
                        }
                    }
                    4 => {
                        // edges format EE-F
                        let mut split = id.split('-');
                        let piece = split.next().unwrap();
                        let face = split.next().unwrap();

                        for edge in &mut cube.edges() {
                            if edge.id() == piece {
                                let (i, _) = edge.pos
                                    .into_iter()
                                    .map(|p| p.as_char())
                                    .enumerate()
                                    .find(|&(_, c)| Some(c) == face.chars().nth(0))
                                    .unwrap();
                                let c = Dom::match_color(&self.colors, &edge.init.pos[i]);

                                if let Some(ref mut fill) = p.fill {
                                    fill.paint = tree::Paint::Color(c);
                                }
                                break;
                            }
                        }
                    }
                    1 => {
                        // centers F
                        for center in &mut cube.centers() {
                            if center.id() == id {
                                let c = Dom::match_color(&self.colors, &center.init.pos);

                                if let Some(ref mut fill) = p.fill {
                                    fill.paint = tree::Paint::Color(c);
                                }
                                break;
                            }
                        }
                    }
                    _n => {} //not a piece
                }
            }
        }
    }
    fn match_color(cs: &ColorScheme, p: &Pos) -> tree::Color {
        match *p {
            Pos::U => cs.u(),
            Pos::D => cs.d(),
            Pos::R => cs.r(),
            Pos::L => cs.l(),
            Pos::F => cs.f(),
            Pos::B => cs.b(),
        }
    }
}

impl ColorScheme {
    pub fn u(&self) -> tree::Color {
        self.u
    }

    pub fn d(&self) -> tree::Color {
        self.d
    }

    pub fn r(&self) -> tree::Color {
        self.r
    }

    pub fn l(&self) -> tree::Color {
        self.l
    }

    pub fn f(&self) -> tree::Color {
        self.f
    }

    pub fn b(&self) -> tree::Color {
        self.b
    }
}

impl default::Default for ColorScheme {
    fn default() -> Self {
        Self {
            u: tree::Color::new(255, 255, 255),
            d: tree::Color::new(255, 255, 0),
            r: tree::Color::new(255, 0, 0),
            l: tree::Color::new(255, 127, 0),
            f: tree::Color::new(0, 255, 31),
            b: tree::Color::new(0, 80, 255),
        }
    }
}
