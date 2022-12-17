use lazy_static::lazy_static;

use crate::generic_cube::{Cube, Face, Move, MoveVariant, CubeSize, all_moves};
use crate::generic_cube::{sticker_index as S};
use crate::facelet_cube::FaceletCube;
use crate::generic_solver::{Solver, PruningTable, ida_star};

/// Solves a 3x3x3 Cube using the Thistlethwaite Algorithm.
///
/// This is the basic 4 phase solver as described on [Jaap's Puzzle Page].
/// The solutions generated are NOT optimal and will take on average 60 moves.
/// While this function will work on a ``GeoCube``, it is highly recommended to
/// use a ``FaceletCube`` for optimal performance.
///
/// [Jaap's Puzzle Page]: https://www.jaapsch.net/puzzles/thistle.htm
///
/// # Examples
///
/// Solve the superflip position:
///
/// ```rust
/// use rubiks_solver::prelude::{Cube};
/// use rubiks_solver::FaceletCube;
/// use rubiks_solver::parse_scramble;
/// use rubiks_solver::solve;
///
/// let cube = FaceletCube::new(3).apply_moves(
///     &parse_scramble(String::from("U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2"))
///  );
//// assert!(cube.apply_moves(&solve(&cube).unwrap()).is_solved());
/// ```
pub fn solve(cube: &impl Cube) -> Option<Vec<Move>> {
    let mut solution = vec![];
    let mut cube = cube.clone();

    for phase in [phase1, phase2, phase3, phase4] {
        let mut phase_solution = phase(&cube)?;
        cube = cube.apply_moves(&phase_solution);
        solution.append(&mut phase_solution);
    }

    Some(solution)
}

pub fn phase1(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;

    lazy_static! {
        static ref MASK: Box<dyn Fn(CubeSize, Face) -> Face + Sync> = {
            let g1_mask = [
                S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
                S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8),
                S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
            ];

            Box::new(move |i: CubeSize, _| if g1_mask.contains(&i) { U } else { X })
        };

        static ref MOVES: Vec<Move> = all_moves(3);

        static ref PRUNING_TABLE: PruningTable = {
            let pruning_depth = 7;
            PruningTable::new(&[FaceletCube::new(3).mask(&*MASK)], pruning_depth, &MOVES)
        };

        static ref SOLVER: Solver = Solver::new(all_moves(3), (*PRUNING_TABLE).clone());
    }

    ida_star(&cube.mask(&*MASK), &*SOLVER, 10)
}

pub fn phase2(cube: &impl Cube) -> Option<Vec<Move>> {
    use Face::*;
    use MoveVariant::*;

    lazy_static! {
        static ref MASK: Box<dyn Fn(CubeSize, Face) -> Face + Sync> = {
            let co_pieces = [
                S(3, U, 1), S(3, U, 3), S(3, U, 7), S(3, U, 9),
                S(3, D, 1), S(3, D, 3), S(3, D, 7), S(3, D, 9)
            ];

            let eo_ud_pieces = [
                S(3, U, 2), S(3, U, 4), S(3, U, 6), S(3, U, 8),
                S(3, D, 2), S(3, D, 4), S(3, D, 6), S(3, D, 8)
            ];

            let eo_e_pieces = [
                S(3, F, 4), S(3, F, 6), S(3, B, 4), S(3, B, 6)
            ];

            Box::new(move |i: CubeSize, _|
                if eo_ud_pieces.contains(&i) || co_pieces.contains(&i) { X }
                else if eo_e_pieces.contains(&i) { U }
                else { R }
            )
        };

        static ref MOVES: Vec<Move> = vec![
            Move::U(Standard), Move::U(Inverse), Move::U(Double),
            Move::D(Standard), Move::D(Inverse), Move::D(Double),
            Move::L(Standard), Move::L(Inverse), Move::L(Double),
            Move::R(Standard), Move::R(Inverse), Move::R(Double),
            Move::F(Double), Move::B(Double)
        ];

        static ref PRUNING_TABLE: PruningTable = {
            let pruning_depth = 5;
            PruningTable::new(&[FaceletCube::new(3).mask(&*MASK)], pruning_depth, &*MOVES)
        };

        static ref SOLVER: Solver = Solver::new((*MOVES).clone(), (*PRUNING_TABLE).clone());
    }

    ida_star(&cube.mask(&*MASK), &*SOLVER, 10)
}

pub fn phase3(cube: &impl Cube) -> Option<Vec<Move>>  {
    use Face::*;
    use MoveVariant::*;

    lazy_static! {
        static ref MASK: Box<dyn Fn(CubeSize, Face) -> Face + Sync> = {
            let cp_pieces = [U, D, F, B, L, R].iter()
                .map(|f| [1, 3, 7, 9].map(|x| S(3, *f, x)))
                .collect::<Vec<_>>()
                .concat();

            let ep_pieces = [F, B, L, R].iter()
                .map(|f| [2, 4, 6, 8].map(|x| S(3, *f, x)))
                .collect::<Vec<_>>()
                .concat();

            let face = |f| if f == B { F }
                           else if f == L { R }
                           else { f };

            Box::new(move |i: CubeSize, _|
                if cp_pieces.contains(&i) { [U, R, F, D, L, B][(i / 9) as usize] }
                else if ep_pieces.contains(&i) { face([U, R, F, D, L, B][(i / 9) as usize]) }
                else { X }
            )
        };

        static ref G2_SOLVED_STATES: PruningTable = PruningTable::new(
            &[FaceletCube::new(3).mask(&*MASK)],
            10,
            &vec![Move::U(Double), Move::D(Double), Move::F(Double), Move::B(Double), Move::L(Double), Move::R(Double)]
        );

        static ref MOVES: Vec<Move> = vec![
            Move::U(Standard), Move::U(Inverse), Move::U(Double),
            Move::D(Standard), Move::D(Inverse), Move::D(Double),
            Move::F(Double), Move::B(Double), Move::L(Double), Move::R(Double)
        ];

        static ref PRUNING_TABLE: PruningTable = {
            let pruning_depth = 5;
            PruningTable::from_existing_table(&G2_SOLVED_STATES, pruning_depth, &*MOVES)
        };

        static ref SOLVER: Solver = Solver::new((*MOVES).clone(), (*PRUNING_TABLE).clone());
    }

    ida_star(&cube.mask(&*MASK), &*SOLVER, 13)
}

pub fn phase4(cube: &impl Cube) -> Option<Vec<Move>> {
    use MoveVariant::*;

    let moves = vec![
        Move::U(Double), Move::D(Double), Move::F(Double),
        Move::B(Double), Move::L(Double), Move::R(Double)
    ];
    let search_limit = 14;
    let pruning_depth = 6;
    let pruning_table = PruningTable::new(&[FaceletCube::new(3)], pruning_depth, &moves);

    let solver = Solver::new(moves, pruning_table);

    ida_star(cube, &solver, search_limit)
}