use rustc_hash::FxHashMap;
use std::mem::discriminant;

use crate::generic_cube::{Cube, Face, Move};
use crate::facelet_cube::FaceletCube;

/// A combination of a Pruning Table and the candidate moves to solve into a specific state.
///
/// This solver packs together the relevant information that can then be passed to a
/// search function to generate cube solutions.
///
/// # Examples
///
/// Thistlethwaite Phase 4 (All Double Moves):
///
/// ```rust
/// use rubiks_solver::prelude::{Cube, Move, MoveVariant::*};
/// use rubiks_solver::FaceletCube;
/// use rubiks_solver::PruningTable;
/// use rubiks_solver::Solver;
///
/// let moves = vec![
///     Move::U(Double), Move::D(Double), Move::F(Double),
///     Move::B(Double), Move::L(Double), Move::R(Double)
/// ];
///
/// let solver = Solver::new(moves.clone(), PruningTable::new(&[FaceletCube::new(3)], 6, &moves));
pub struct Solver {
    pub candidate_moves: Vec<Move>,
    pub pruning_table: PruningTable
}

#[derive(Clone)]
/// A Pruning Table giving a lower bound for the number of moves to solve a specific state.
///
/// A Pruning Table is essential for the IDA* algorithm in order to allow for tree pruning during
/// the iterative deepening depth first search. If at any point during a search we reach a state
/// that would take too many moves to solve, we can abandon this search branch, greatly reducing
/// our search space.
pub struct PruningTable {
    pruning_table: FxHashMap<Vec<Face>, i32>,
    depth: i32
}

impl PruningTable {
    /// Constructs a Pruning Table given the set of starting cubes, pruning depth and allowable moves.
    ///
    /// # Examples
    ///
    /// Thistlethwaite Phase 4 (All Double Moves):
    ///
    /// ```rust
    /// use rubiks_solver::prelude::{Cube, Move, MoveVariant::*};
    /// use rubiks_solver::FaceletCube;
    /// use rubiks_solver::PruningTable;
    ///
    /// let moves = vec![
    ///     Move::U(Double), Move::D(Double), Move::F(Double),
    ///     Move::B(Double), Move::L(Double), Move::R(Double)
    /// ];
    ///
    /// let pruning_table = PruningTable::new(&[FaceletCube::new(3)], 6, &moves);
    /// ```
    pub fn new(starting_cubes: &[impl Cube], depth: i32, moveset: &[Move]) -> Self {
        let mut pruning_table: FxHashMap<Vec<Face>, i32> = FxHashMap::default();
        let mut previous_frontier = starting_cubes.to_vec();

        for cube in starting_cubes {
            pruning_table.insert(cube.state(), 0);
        }

        for i in 1..=depth {
            let mut frontier = vec![];

            for cube in previous_frontier {
                for mv in moveset {
                    let new_cube = cube.apply_move(*mv);
                    if let std::collections::hash_map::Entry::Vacant(e) = pruning_table.entry(new_cube.state()) {
                        e.insert(i);
                        frontier.push(new_cube);
                    }
                }
            }

            previous_frontier = frontier.clone();
        }

        Self {
            pruning_table,
            depth
        }
    }

    /// Constructs a Pruning Table using all the states in an existing Pruning Table as start states.
    pub fn from_existing_table(other: &PruningTable, depth: i32, moveset: &[Move]) -> Self {
        Self::new(
            &other.pruning_table.keys().map(|faces| FaceletCube::from(faces.clone())).collect::<Vec<_>>(),
            depth,
            moveset
        )
    }

    fn get(&self, k: &[Face]) -> Option<&i32> {
        self.pruning_table.get(k)
    }
}

impl Solver {
    pub fn new(candidate_moves: Vec<Move>, pruning_table: PruningTable) -> Self {
        Self {
            candidate_moves,
            pruning_table
        }
    }

    pub fn is_solved(&self, cube: &impl Cube) -> bool {
        matches!(self.pruning_table.get(&cube.state()), Some(0))
    }

    pub fn lower_bound(&self, cube: &impl Cube) -> i32 {
        match self.pruning_table.get(&cube.state()) {
            Some(n) => *n,
            _ => self.pruning_table.depth + 1
        }
    }
}

pub fn ida_star(cube: &impl Cube,
                solver: &Solver,
                limit: i32) -> Option<Vec<Move>> {
    for i in 0..=limit {
        if let Some(sol) = dfs(cube, solver, &mut vec![], i) {
            return Some(sol);
        }
    }

    None
}

fn dfs(cube: &impl Cube,
       solver: &Solver,
       solution: &mut Vec<Move>,
       depth_remaining: i32) -> Option<Vec<Move>> {
    if solver.is_solved(cube) {
        return Some(solution.to_vec());
    }

    if solver.lower_bound(cube) > depth_remaining {
        return None;
    }

    for mv in &solver.candidate_moves {
        if let Some(last_mv) = solution.last() {
            if discriminant(last_mv) == discriminant(mv) {
                continue;
            }
        }

        solution.push(*mv);

        let result = dfs(
            &cube.apply_move(*mv),
            solver,
            solution,
            depth_remaining - 1
        );

        solution.pop();

        if result.is_some() {
            return result;
        }
    }

    None
}