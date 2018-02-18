extern crate rscube;

use rscube::alg::parse::Parser;
use rscube::cube::Cube;
use rscube::render::{Dom, WriteBuffer};

use std::io::{BufRead, Write};
use std::fs::File;

const RUBIKS_CUBE: &'static str = "gfx/rubiks_cube.svg";
const RUBIKS_CUBE_MODIF: &'static str = "gfx/modif_rubiks_cube.svg";

fn main() {
    let mut dom = Dom::load(RUBIKS_CUBE);
    let mut cube = Cube::new();
    let mut parser = Parser::new();

    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();

        match line.as_ref() {
            ":q" => break,
            ":r" => {
                cube = Cube::new();
                continue;
            }
            _ => {
                if let Some(turns) = parser.parse(&line).report().generate() {
                    for &t in &turns {
                        cube.turn(t);
                    }
                }
            }
        }
        dom.update(&cube);

        let mut ouput_data = Vec::new();
        dom.doc.write_buf(&mut ouput_data);

        let mut f = File::create(RUBIKS_CUBE_MODIF).unwrap();
        f.write_all(&ouput_data).unwrap();
    }
}
