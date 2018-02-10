extern crate rscube;

use rscube::cube::Cube;
use rscube::cube::turns::Turn::*;

fn main() {
    // For now just test it with an H-perm
    let hperm = [M, M, U, M, M, U, U, M, M, U, M, M];
    let mut cube = Cube::new();

    for &t in &hperm {
        cube.turn(t)
    }

    // only the top edges should be unsolved
    cube.list_any(|cubie| !cubie.is_solved())
}
