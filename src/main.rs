extern crate rscube;

use rscube::alg::parse::Parser;
use rscube::cube::Cube;
use rscube::cube::cubie::Cubie;
use std::io::BufRead;

fn main() {
    let mut cube = Cube::new();
    let mut parser = Parser::new();
    let stdin = std::io::stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if line == ":q" { break }
        if line == ":r" { cube = Cube::new(); continue }

        let turns = parser.parse(&line)
            .report()
            .generate();
        if turns.is_some() {
            let turns = turns.unwrap();
            for &t in &turns {
                cube.turn(t);
            }
        }
        cube.list_any(|x| !x.is_solved())
    }
}
