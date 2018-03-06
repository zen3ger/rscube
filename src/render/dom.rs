use resvg::{self, tree};
use std::default;

use cube::Cube;
use cube::cubie::Pos;

pub struct Dom {
    pub rtree: resvg::tree::RenderTree,
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
        () //do nothing for now
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
