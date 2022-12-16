use crate::generic_cube::Face::*;
use crate::generic_cube::{Cube, CubeSize, Face, Move};

use super::moves::compute_permutation;

/// A Rubik's Cube with stickers stored sequentially in a 1-dimensional array.
///
/// Each move is implemented as an array of index mappings of the form ``(from_idx, to_idx)``.
/// A move is then applied by swapping all pieces as specified by these index mappings.
///
/// Applying moves for the ``FaceletCube`` is more efficient than the ``GeoCube``, but
/// it is harder to define moves from scratch. Instead of deriving index mappings from scratch,
/// we first implement a GeoCube move, then use our conversion function to map the move
/// to a FaceletCube move.
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct FaceletCube {
    size: CubeSize,
    faces: Vec<(Face, u16)>,
}

impl Cube for FaceletCube {
    fn new(size: CubeSize) -> Self {
        Self {
            size,
            faces: vec![
                repeat(U, (size * size).into()),
                repeat(R, (size * size).into()),
                repeat(F, (size * size).into()),
                repeat(D, (size * size).into()),
                repeat(L, (size * size).into()),
                repeat(B, (size * size).into()),
            ]
            .concat()
            .iter()
            .enumerate()
            .map(|(i, s)| (*s, i as u16))
            .collect(),
        }
    }

    fn size(&self) -> CubeSize {
        self.size
    }

    fn state(&self) -> Vec<Face> {
        self.faces.iter().map(|(s, _)| *s).collect()
    }

    fn mask(&self, mask: &dyn Fn(CubeSize, Face) -> Face) -> Self {
        let masked_faces = self
            .faces
            .iter()
            .map(|(f, i)| (mask(*i as CubeSize, *f), *i))
            .collect();

        Self {
            faces: masked_faces,
            ..*self
        }
    }

    fn apply_move(&self, mv: Move) -> Self {
        Self {
            size: self.size,
            faces: compute_permutation(&self.faces, self.size, mv),
        }
    }
}

impl From<Vec<Face>> for FaceletCube {
    fn from(faces: Vec<Face>) -> FaceletCube {
        FaceletCube {
            size: ((faces.len() / 6) as f64).sqrt() as CubeSize,
            faces: faces.iter().map(|f| (*f, 0)).collect(),
        }
    }
}

fn repeat<T: Clone>(element: T, count: i32) -> Vec<T> {
    std::iter::repeat(element).take(count as usize).collect()
}
