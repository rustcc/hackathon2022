//
// 4x4x4 Tests
//

use test_case::test_case;

use rubiks_solver::prelude::Face::*;
use rubiks_solver::prelude::*;
use rubiks_solver::solved_state;
use rubiks_solver::{FaceletCube, GeoCube};

//
// State Tests
//
#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn solved_state(cube: impl Cube) {
    assert_eq!(
        cube.state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn u_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::U(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, B, B, B, R, R, R, R, R, R, R, R, R,
            R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, F, F, F, F, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn uw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Uw(2, MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, B, B, B, B, B, B, B, B, R, R, R, R, R,
            R, R, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, F, F, F, F, F, F, F, F, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L,
            L, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn r_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::R(MoveVariant::Standard)).state(),
        vec![
            U, U, U, F, U, U, U, F, U, U, U, F, U, U, U, F, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, F, F, F, D, F, F, F, D, F, F, F, D, F, F, F, D, D, D, D, B, D, D, D, B, D, D,
            D, B, D, D, D, B, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, U, B, B, B, U, B, B,
            B, U, B, B, B, U, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn rw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Rw(2, MoveVariant::Standard)).state(),
        vec![
            U, U, F, F, U, U, F, F, U, U, F, F, U, U, F, F, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, F, F, D, D, F, F, D, D, F, F, D, D, F, F, D, D, D, D, B, B, D, D, B, B, D, D,
            B, B, D, D, B, B, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, U, U, B, B, U, U, B,
            B, U, U, B, B, U, U, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn f_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::F(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, L, L, L, L, U, R, R, R, U, R, R, R, U, R, R, R, U,
            R, R, R, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, R, R, R, R, D, D, D, D, D, D,
            D, D, D, D, D, D, L, L, L, D, L, L, L, D, L, L, L, D, L, L, L, D, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn fw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Fw(2, MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, L, L, L, L, L, L, L, L, U, U, R, R, U, U, R, R, U, U, R, R, U,
            U, R, R, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, R, R, R, R, R, R, R, R, D, D,
            D, D, D, D, D, D, L, L, D, D, L, L, D, D, L, L, D, D, L, L, D, D, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn l_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::L(MoveVariant::Standard)).state(),
        vec![
            B, U, U, U, B, U, U, U, B, U, U, U, B, U, U, U, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, U, F, F, F, U, F, F, F, U, F, F, F, U, F, F, F, F, D, D, D, F, D, D, D, F, D,
            D, D, F, D, D, D, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, B, B, B, D, B, B, B,
            D, B, B, B, D, B, B, B, D
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn lw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Lw(2, MoveVariant::Standard)).state(),
        vec![
            B, B, U, U, B, B, U, U, B, B, U, U, B, B, U, U, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, U, U, F, F, U, U, F, F, U, U, F, F, U, U, F, F, F, F, D, D, F, F, D, D, F, F,
            D, D, F, F, D, D, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, B, B, D, D, B, B, D,
            D, B, B, D, D, B, B, D, D
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn d_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::D(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, R, R, R, F,
            F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, L, L, L, L, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B, B, B,
            B, B, B, B, B, R, R, R, R
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn dw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Dw(2, MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, F, F, F, F, F,
            F, F, F, F, F, F, F, F, F, F, F, L, L, L, L, L, L, L, L, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B, B, B, B, B, B, B,
            B, R, R, R, R, R, R, R, R
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn b_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::B(MoveVariant::Standard)).state(),
        vec![
            R, R, R, R, U, U, U, U, U, U, U, U, U, U, U, U, R, R, R, D, R, R, R, D, R, R, R, D, R,
            R, R, D, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, D, D,
            D, D, L, L, L, L, U, L, L, L, U, L, L, L, U, L, L, L, U, L, L, L, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(4) ; "Geometric Cube")]
#[test_case(FaceletCube::new(4) ; "Facelet Cube")]
fn bw2_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::Bw(2, MoveVariant::Standard)).state(),
        vec![
            R, R, R, R, R, R, R, R, U, U, U, U, U, U, U, U, R, R, D, D, R, R, D, D, R, R, D, D, R,
            R, D, D, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, L, L,
            L, L, L, L, L, L, U, U, L, L, U, U, L, L, U, U, L, L, U, U, L, L, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test]
fn generic_solved_state() {
    assert_eq!(
        solved_state(4),
        vec![
            U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, R, R, R, R,
            R, R, R, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, D, D, D, D, D, D, D, D, D, D,
            D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B,
            B, B, B, B, B, B, B, B, B
        ]
    );
}
