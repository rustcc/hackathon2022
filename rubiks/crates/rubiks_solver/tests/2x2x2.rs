//
// 2x2x2 Tests
//

use test_case::test_case;

use rubiks_solver::prelude::Face::*;
use rubiks_solver::prelude::*;
use rubiks_solver::solved_state;
use rubiks_solver::{FaceletCube, GeoCube};

//
// State Tests
//
#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn solved_state(cube: impl Cube) {
    assert_eq!(
        cube.state(),
        vec![U, U, U, U, R, R, R, R, F, F, F, F, D, D, D, D, L, L, L, L, B, B, B, B,]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn u_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::U(MoveVariant::Standard)).state(),
        vec![U, U, U, U, B, B, R, R, R, R, F, F, D, D, D, D, F, F, L, L, L, L, B, B]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn r_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::R(MoveVariant::Standard)).state(),
        vec![U, F, U, F, R, R, R, R, F, D, F, D, D, B, D, B, L, L, L, L, U, B, U, B]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn f_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::F(MoveVariant::Standard)).state(),
        vec![U, U, L, L, U, R, U, R, F, F, F, F, R, R, D, D, L, D, L, D, B, B, B, B]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn d_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::D(MoveVariant::Standard)).state(),
        vec![U, U, U, U, R, R, F, F, F, F, L, L, D, D, D, D, L, L, B, B, B, B, R, R]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn l_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::L(MoveVariant::Standard)).state(),
        vec![B, U, B, U, R, R, R, R, U, F, U, F, F, D, F, D, L, L, L, L, B, D, B, D]
    );
}

#[test_case(GeoCube::new(2) ; "Geometric Cube")]
#[test_case(FaceletCube::new(2) ; "Facelet Cube")]
fn b_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::B(MoveVariant::Standard)).state(),
        vec![R, R, U, U, R, D, R, D, F, F, F, F, D, D, L, L, U, L, U, L, B, B, B, B]
    );
}

#[test]
fn generic_solved_state() {
    assert_eq!(
        solved_state(2),
        vec![U, U, U, U, R, R, R, R, F, F, F, F, D, D, D, D, L, L, L, L, B, B, B, B]
    );
}
