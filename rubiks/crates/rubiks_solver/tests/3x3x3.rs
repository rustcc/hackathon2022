//
// 3x3x3 Tests
//

use test_case::test_case;

use rubiks_solver::prelude::Face::*;
use rubiks_solver::prelude::*;
use rubiks_solver::solved_state;
use rubiks_solver::{FaceletCube, GeoCube};

//
// State Tests
//
#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn solved_state(cube: impl Cube) {
    assert_eq!(
        cube.state(),
        vec![
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn u_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::U(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, B, B, B, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, F, F, F, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn r_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::R(MoveVariant::Standard)).state(),
        vec![
            U, U, F, U, U, F, U, U, F, R, R, R, R, R, R, R, R, R, F, F, D, F, F, D, F, F, D, D, D,
            B, D, D, B, D, D, B, L, L, L, L, L, L, L, L, L, U, B, B, U, B, B, U, B, B
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn f_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::F(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, L, L, L, U, R, R, U, R, R, U, R, R, F, F, F, F, F, F, F, F, F, R, R,
            R, D, D, D, D, D, D, L, L, D, L, L, D, L, L, D, B, B, B, B, B, B, B, B, B
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn l_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::L(MoveVariant::Standard)).state(),
        vec![
            B, U, U, B, U, U, B, U, U, R, R, R, R, R, R, R, R, R, U, F, F, U, F, F, U, F, F, F, D,
            D, F, D, D, F, D, D, L, L, L, L, L, L, L, L, L, B, B, D, B, B, D, B, B, D
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn d_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::D(MoveVariant::Standard)).state(),
        vec![
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, L, L, L, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B, R, R, R
        ]
    );
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn b_move(cube: impl Cube) {
    assert_eq!(
        cube.apply_move(Move::B(MoveVariant::Standard)).state(),
        vec![
            R, R, R, U, U, U, U, U, U, R, R, D, R, R, D, R, R, D, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, L, L, L, U, L, L, U, L, L, U, L, L, B, B, B, B, B, B, B, B, B
        ]
    );
}

//
// Is Solved Tests
//
#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn new_cube_is_solved(cube: impl Cube) {
    assert!(cube.is_solved());
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn rotated_cube_is_solved(cube: impl Cube) {
    use Move::*;
    use MoveVariant::*;

    assert!(cube.apply_move(X(Standard)).is_solved());
    assert!(cube.apply_move(Y(Standard)).is_solved());
    assert!(cube.apply_move(Z(Standard)).is_solved());

    assert!(cube.apply_move(X(Inverse)).is_solved());
    assert!(cube.apply_move(Y(Inverse)).is_solved());
    assert!(cube.apply_move(Z(Inverse)).is_solved());

    assert!(cube.apply_move(X(Double)).is_solved());
    assert!(cube.apply_move(Y(Double)).is_solved());
    assert!(cube.apply_move(Z(Double)).is_solved());
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn single_move_cube_is_not_solved(cube: impl Cube) {
    use Move::*;
    use MoveVariant::*;

    assert!(!cube.apply_move(U(Standard)).is_solved());
    assert!(!cube.apply_move(R(Standard)).is_solved());
    assert!(!cube.apply_move(F(Standard)).is_solved());
    assert!(!cube.apply_move(D(Standard)).is_solved());
    assert!(!cube.apply_move(L(Standard)).is_solved());
    assert!(!cube.apply_move(B(Standard)).is_solved());
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn standard_inverse_move_cube_is_solved(cube: impl Cube) {
    use Move::*;
    use MoveVariant::*;

    assert!(cube
        .apply_move(U(Standard))
        .apply_move(U(Inverse))
        .is_solved());
    assert!(cube
        .apply_move(R(Standard))
        .apply_move(R(Inverse))
        .is_solved());
    assert!(cube
        .apply_move(F(Standard))
        .apply_move(F(Inverse))
        .is_solved());
    assert!(cube
        .apply_move(D(Standard))
        .apply_move(D(Inverse))
        .is_solved());
    assert!(cube
        .apply_move(L(Standard))
        .apply_move(L(Inverse))
        .is_solved());
    assert!(cube
        .apply_move(B(Standard))
        .apply_move(B(Inverse))
        .is_solved());
}

#[test_case(GeoCube::new(3) ; "Geometric Cube")]
#[test_case(FaceletCube::new(3) ; "Facelet Cube")]
fn double_double_move_cube_is_solved(cube: impl Cube) {
    use Move::*;
    use MoveVariant::*;

    assert!(cube.apply_move(U(Double)).apply_move(U(Double)).is_solved());
    assert!(cube.apply_move(R(Double)).apply_move(R(Double)).is_solved());
    assert!(cube.apply_move(F(Double)).apply_move(F(Double)).is_solved());
    assert!(cube.apply_move(D(Double)).apply_move(D(Double)).is_solved());
    assert!(cube.apply_move(L(Double)).apply_move(L(Double)).is_solved());
    assert!(cube.apply_move(B(Double)).apply_move(B(Double)).is_solved());
}

#[test]
fn generic_solved_state() {
    assert_eq!(
        solved_state(3),
        vec![
            U, U, U, U, U, U, U, U, U, R, R, R, R, R, R, R, R, R, F, F, F, F, F, F, F, F, F, D, D,
            D, D, D, D, D, D, D, L, L, L, L, L, L, L, L, L, B, B, B, B, B, B, B, B, B
        ]
    );
}
