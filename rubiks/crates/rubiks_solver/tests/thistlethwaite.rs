//
// Thistlethwaite Solver Tests
//
// We only test the Facelet Cube due to performance considerations
//

use rubiks_solver::parse_scramble;
use rubiks_solver::prelude::*;
use rubiks_solver::solve;
use rubiks_solver::FaceletCube;

#[test]
fn solved_state() {
    let cube = FaceletCube::new(3);
    assert!(cube.apply_moves(&solve(&cube).unwrap()).is_solved());
}

#[test]
fn single_move() {
    let cube = FaceletCube::new(3).apply_move(Move::U(MoveVariant::Standard));
    assert!(cube.apply_moves(&solve(&cube).unwrap()).is_solved());
}

#[test]
fn longer_scramble() {
    let cube = FaceletCube::new(3).apply_moves(&parse_scramble(String::from("U F R2 D2 B2 L R")));
    assert!(cube.apply_moves(&solve(&cube).unwrap()).is_solved());
}

#[test]
fn superflip() {
    let cube = FaceletCube::new(3).apply_moves(&parse_scramble(String::from(
        "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2",
    )));
    assert!(cube.apply_moves(&solve(&cube).unwrap()).is_solved());
}
