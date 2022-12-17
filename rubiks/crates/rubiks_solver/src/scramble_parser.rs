use crate::generic_cube::{Move, MoveVariant, CubeSize};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

/// Converts a WCA Notation scramble into ``Vec<Move>``.
pub fn parse_scramble(scramble: String) -> Vec<Move> {
    scramble.trim().split_whitespace().map(convert_move).collect()
}

fn convert_move(mv: &str) -> Move {
    let slice = get_slice(mv);
    let variant = get_variant(mv);

    if !mv.contains('w') {
        match &mv[0..1] {
            "U" => U(variant),
            "R" => R(variant),
            "F" => F(variant),
            "L" => L(variant),
            "D" => D(variant),
            "B" => B(variant),
            "x" => X(variant),
            "y" => Y(variant),
            "z" => Z(variant),
            _ => panic!()
        }
    } else if mv.contains('U') {
        Uw(slice, variant)
    } else if mv.contains('R') {
        Rw(slice, variant)
    } else if mv.contains('F') {
        Fw(slice, variant)
    } else if mv.contains('L') {
        Lw(slice, variant)
    } else if mv.contains('D') {
        Dw(slice, variant)
    } else if mv.contains('B') {
        Bw(slice, variant)
    } else if mv.contains('x') {
        X(variant)
    } else if mv.contains('y') {
        Y(variant)
    } else if mv.contains('z') {
        Z(variant)
    } else {
        panic!()
    }
}

fn get_slice(mv: &str) -> CubeSize {
    if !mv.contains('w') {
        1
    } else {
        mv[0..1].parse::<CubeSize>().unwrap_or(2)
    }
}

fn get_variant(mv: &str) -> MoveVariant {
    if mv.contains('2') {
        Double
    } else if mv.contains('\'') {
        Inverse
    } else {
        Standard
    }
}

/// Recursively merges adjacent moves with the same Move type
/// until no further simplification is possible.
///
/// # Examples
///
/// Simplifying some scrambles:
///
/// ```rust
/// use rubiks_solver::{parse_scramble, simplify_moves};
/// use rubiks_solver::prelude::{Move::*, MoveVariant::*};
///
/// let scramble = parse_scramble(String::from("B B2 B' R B2 B' R2 R' F2"));
/// let simplified = simplify_moves(&scramble);
/// assert_eq!(simplified, vec![B(Double), R(Standard), B(Standard), R(Standard), F(Double)]);
///
/// let scramble = parse_scramble(String::from("R R' U2 F F' U2 x"));
/// let simplified = simplify_moves(&scramble);
/// assert_eq!(simplified, vec![X(Standard)]);
/// ```
pub fn simplify_moves(moves: &[Move]) -> Vec<Move> {
    // Recursively merges adjacent moves of the same Move discriminant
    // until no further simplification is possible.
    use std::mem::discriminant;
    let mut result = vec![];
    if moves.is_empty() {
        return result;
    }

    // keep track of the current move and its amount of clockwise turns
    struct Movement { pub mv: Move, pub total_turns: u8 }
    let mut movement: Movement = Movement { mv: moves[0], total_turns: moves[0].get_variant() as u8};

    // returns a Move if the simplified movement has any effect on a cube
    fn movement_to_move(m: Movement) -> Option<Move> {
        match m.total_turns % 4 {
            1 => Some(m.mv.with_variant(MoveVariant::Standard)),
            2 => Some(m.mv.with_variant(MoveVariant::Double)),
            3 => Some(m.mv.with_variant(MoveVariant::Inverse)),
            _ => None,
        }
    }

    // merge adjacent moves of the same type
    for mv in moves[1..].iter() {
        if discriminant(&movement.mv) == discriminant(mv) {
            movement.total_turns = (movement.total_turns + mv.get_variant() as u8) % 4;
        } else {
            if let Some(m) = movement_to_move(movement) { result.push(m) };
            movement = Movement { mv: *mv, total_turns: mv.get_variant() as u8 };
        }
    }
    if let Some(m) = movement_to_move(movement) { result.push(m) };

    // don't recurse if moves couldn't be simplified further
    if result.len() == moves.len() { return result }
    simplify_moves(result.as_slice())
}