//
// Move Parser Tests
//

use rubiks_solver::prelude::{Move::*, MoveVariant::*};
use rubiks_solver::{parse_scramble, simplify_moves};

#[test]
fn test_basic_moves() {
    assert_eq!(parse_scramble(String::from("U"))[0], U(Standard));
    assert_eq!(parse_scramble(String::from("F"))[0], F(Standard));
    assert_eq!(parse_scramble(String::from("R"))[0], R(Standard));
    assert_eq!(parse_scramble(String::from("D"))[0], D(Standard));
    assert_eq!(parse_scramble(String::from("L"))[0], L(Standard));
    assert_eq!(parse_scramble(String::from("B"))[0], B(Standard));

    assert_eq!(parse_scramble(String::from("x"))[0], X(Standard));
    assert_eq!(parse_scramble(String::from("y"))[0], Y(Standard));
    assert_eq!(parse_scramble(String::from("z"))[0], Z(Standard));
}

#[test]
fn test_double_moves() {
    assert_eq!(parse_scramble(String::from("U2"))[0], U(Double));
    assert_eq!(parse_scramble(String::from("F2"))[0], F(Double));
    assert_eq!(parse_scramble(String::from("R2"))[0], R(Double));
    assert_eq!(parse_scramble(String::from("D2"))[0], D(Double));
    assert_eq!(parse_scramble(String::from("L2"))[0], L(Double));
    assert_eq!(parse_scramble(String::from("B2"))[0], B(Double));

    assert_eq!(parse_scramble(String::from("x2"))[0], X(Double));
    assert_eq!(parse_scramble(String::from("y2"))[0], Y(Double));
    assert_eq!(parse_scramble(String::from("z2"))[0], Z(Double));
}

#[test]
fn test_inverse_moves() {
    assert_eq!(parse_scramble(String::from("U'"))[0], U(Inverse));
    assert_eq!(parse_scramble(String::from("F'"))[0], F(Inverse));
    assert_eq!(parse_scramble(String::from("R'"))[0], R(Inverse));
    assert_eq!(parse_scramble(String::from("D'"))[0], D(Inverse));
    assert_eq!(parse_scramble(String::from("L'"))[0], L(Inverse));
    assert_eq!(parse_scramble(String::from("B'"))[0], B(Inverse));

    assert_eq!(parse_scramble(String::from("x'"))[0], X(Inverse));
    assert_eq!(parse_scramble(String::from("y'"))[0], Y(Inverse));
    assert_eq!(parse_scramble(String::from("z'"))[0], Z(Inverse));
}

#[test]
fn test_long_scramble() {
    assert_eq!(
        parse_scramble(String::from("R U R' U' R' F R2 U' R' U' R U R' F'")),
        vec![
            R(Standard),
            U(Standard),
            R(Inverse),
            U(Inverse),
            R(Inverse),
            F(Standard),
            R(Double),
            U(Inverse),
            R(Inverse),
            U(Inverse),
            R(Standard),
            U(Standard),
            R(Inverse),
            F(Inverse),
        ]
    );
}

#[test]
fn test_wide_moves() {
    assert_eq!(
        parse_scramble(String::from("Rw 3Fw 5Bw' 3Lw2")),
        vec![
            Rw(2, Standard),
            Fw(3, Standard),
            Bw(5, Inverse),
            Lw(3, Double)
        ]
    );
}

#[test]
fn test_simplify_unsimplifiable_moves() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("x2"))),
        vec![X(Double)]
    );
    assert_eq!(simplify_moves(&parse_scramble(String::from(""))), vec![]);
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("x y z"))),
        vec![X(Standard), Y(Standard), Z(Standard)]
    );
}

#[test]
fn test_simplify_standard_and_inverse_cancel() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("U U'"))),
        vec![]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("R' R"))),
        vec![]
    );
}

#[test]
fn test_simplify_two_doubles_cancel() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("D2 D2"))),
        vec![]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("2Lw 2Lw"))),
        vec![]
    );
}

#[test]
fn test_simplify_double_and_standard_becomes_inverse() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("L2 L"))),
        vec![L(Inverse)]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("F F2"))),
        vec![F(Inverse)]
    );
}

#[test]
fn test_simplify_double_and_inverse_becomes_standard() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("L2 L'"))),
        vec![L(Standard)]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("F' F2"))),
        vec![F(Standard)]
    );
}

#[test]
fn test_simplify_two_singles_becomes_double() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("B B"))),
        vec![B(Double)]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("Uw Uw"))),
        vec![Uw(2, Double)]
    );

    assert_eq!(
        simplify_moves(&parse_scramble(String::from("B' B'"))),
        vec![B(Double)]
    );
    assert_eq!(
        simplify_moves(&parse_scramble(String::from("Uw' Uw'"))),
        vec![Uw(2, Double)]
    );
}

#[test]
fn test_simplify_long() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from(
            "B B2 B' R B2 B' R2 R' F2 R' R'"
        ))),
        vec![
            B(Double),
            R(Standard),
            B(Standard),
            R(Standard),
            F(Double),
            R(Double)
        ]
    );
}

#[test]
fn test_simplify_complex() {
    assert_eq!(
        simplify_moves(&parse_scramble(String::from(
            "R U2 R' R U2 F F' U2 B2 B2 U' U2 U R'"
        ))),
        vec![]
    )
}
