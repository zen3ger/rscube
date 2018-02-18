use render::{svgdom, ColorScheme};
use cube::Cube;
use cube::cubie::Pos;

use std::fs::File;
use std::io::Read;
use std::default::Default;

pub struct Dom {
    pub doc: svgdom::Document,
    colors: ColorScheme,
}

impl Dom {
    pub fn load(path: &str) -> Self {
        let mut file = File::open(path).expect("Unable to load svg");
        let length = file.metadata().unwrap().len() as usize;

        let mut inp_data = String::with_capacity(length + 1);
        file.read_to_string(&mut inp_data).unwrap();

        let doc = svgdom::Document::from_str(&inp_data).expect("Failed to create DOM");

        Dom {
            doc: doc,
            colors: ColorScheme::default(),
        }
    }

    pub fn update(&mut self, cube: &Cube) {
        let root = self.doc.first_child();
        if root.is_none() {
            return;
        }

        let root = root.unwrap();
        for child in root.children() {
            let id = child.id().clone();
            match id.as_ref() {
                "CORNERS" => for (node, corner) in child.children().zip(cube.corners()) {
                    for (pos, mut rect) in corner.pos.into_iter().zip(node.children()) {
                        let color = match pos {
                            Pos::U => self.colors.u(),
                            Pos::D => self.colors.d(),
                            Pos::R => self.colors.r(),
                            Pos::L => self.colors.l(),
                            Pos::F => self.colors.f(),
                            Pos::B => self.colors.b(),
                        };
                        rect.set_attribute((svgdom::AttributeId::Fill, color));
                    }
                },
                "EDGES" => for (node, edge) in child.children().zip(cube.edges()) {
                    for (pos, mut rect) in edge.pos.into_iter().zip(node.children()) {
                        let color = match pos {
                            Pos::U => self.colors.u(),
                            Pos::D => self.colors.d(),
                            Pos::R => self.colors.r(),
                            Pos::L => self.colors.l(),
                            Pos::F => self.colors.f(),
                            Pos::B => self.colors.b(),
                        };
                        rect.set_attribute((svgdom::AttributeId::Fill, color));
                    }
                },
                "CENTERS" => for (mut rect, center) in child.children().zip(cube.centers()) {
                    let color = match center.pos() {
                        Pos::U => self.colors.u(),
                        Pos::D => self.colors.d(),
                        Pos::R => self.colors.r(),
                        Pos::L => self.colors.l(),
                        Pos::F => self.colors.f(),
                        Pos::B => self.colors.b(),
                    };
                    rect.set_attribute((svgdom::AttributeId::Fill, color));
                },
                _ => continue,
            }
        }
    }
}
