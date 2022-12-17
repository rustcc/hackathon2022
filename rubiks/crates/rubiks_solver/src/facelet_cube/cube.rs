use crate::generic_cube::Face::*;
use crate::generic_cube::{Cube, CubeSize, Face, Move};

use super::moves::compute_permutation;

///  将魔方的贴纸用一维数组存储起来，数组的长度为 6 * n^2，其中 n 为魔方的阶数。
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
