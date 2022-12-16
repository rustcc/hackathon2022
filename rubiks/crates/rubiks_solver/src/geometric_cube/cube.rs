use lazy_static::lazy_static;

use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::generic_cube::{Cube, CubeSize, Face, Move};

use super::moves::GeometricMove;
use super::sticker::Sticker;

lazy_static! {
    static ref FACE_ROTATING_MOVES: Vec<Vec<Move>> = vec![
        vec![],
        vec![Y(Standard), X(Standard)],
        vec![X(Standard)],
        vec![X(Double)],
        vec![Y(Inverse), X(Standard)],
        vec![Y(Double), X(Standard)],
    ];
}

/// 基础的魔方, 每个面都是贴纸, 通过向量和矩阵旋转来转动
///
/// 编程方便，但是效率低下
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct GeoCube {
    pub(crate) size: CubeSize,
    pub(crate) stickers: Vec<(Sticker, CubeSize)>,
}

impl Cube for GeoCube {
    fn new(size: CubeSize) -> Self {
        Self {
            size,
            stickers: Self::generate_stickers(size),
        }
    }

    fn size(&self) -> CubeSize {
        self.size
    }

    fn state(&self) -> Vec<Face> {
        let mut faces = Vec::new();

        for mvs in FACE_ROTATING_MOVES.iter() {
            let rotated_cube = self.apply_moves(&mvs);
            let top_layer_stickers = rotated_cube.top_layer_stickers();

            for (sticker, _) in top_layer_stickers {
                faces.push(sticker.initial_face());
            }
        }

        faces
    }

    fn mask(&self, mask: &dyn Fn(CubeSize, Face) -> Face) -> Self {
        let masked_stickers = self
            .stickers
            .iter()
            .map(|(s, i)| {
                (
                    Sticker {
                        face: mask(*i, s.initial_face()),
                        ..*s
                    },
                    *i,
                )
            })
            .collect::<Vec<_>>();

        Self {
            stickers: masked_stickers,
            ..*self
        }
    }

    fn apply_move(&self, mv: Move) -> Self {
        Self {
            stickers: self
                .stickers
                .iter()
                .map(|(s, i)| (s.rotate(GeometricMove::from(mv)), *i))
                .collect(),
            ..self.clone()
        }
    }
}

impl GeoCube {
    fn generate_stickers(size: CubeSize) -> Vec<(Sticker, CubeSize)> {
        let mut stickers = Vec::new();

        for face in [-size, size] {
            for p1 in Self::range(size) {
                for p2 in Self::range(size) {
                    stickers.push((Sticker::new(size, face, p1, p2), -1));
                    stickers.push((Sticker::new(size, p1, face, p2), -1));
                    stickers.push((Sticker::new(size, p1, p2, face), -1));
                }
            }
        }

        Self::set_sticker_initial_index(size, stickers)
    }

    fn set_sticker_initial_index(
        size: CubeSize,
        stickers: Vec<(Sticker, CubeSize)>,
    ) -> Vec<(Sticker, CubeSize)> {
        let mut result = Vec::new();
        let cube = Self { size, stickers };

        for (idx, mvs) in (FACE_ROTATING_MOVES).iter().enumerate() {
            let rotated_cube = cube.apply_moves(&mvs);
            let top_layer_stickers = rotated_cube.top_layer_stickers();

            for (sticker, _) in top_layer_stickers.iter() {
                result.push((sticker.set_solved(), idx as CubeSize));
            }
        }

        result
    }

    fn top_layer_stickers(&self) -> Vec<(Sticker, CubeSize)> {
        let mut top_layer_stickers = self
            .stickers
            .to_owned()
            .into_iter()
            .filter(|(s, _)| matches!(s.current_face(), Face::U))
            .collect::<Vec<_>>();

        top_layer_stickers.sort_by_key(|(s, _)| (s.current.z as CubeSize, s.current.x as CubeSize));
        top_layer_stickers
    }

    /// Returns the range of facelet center coordinates along an arbitrary axis.
    pub fn range(size: CubeSize) -> Vec<CubeSize> {
        (-size + 1..=size - 1).step_by(2).collect()
    }

    pub fn stickers(&self) -> Vec<Sticker> {
        self.stickers.iter().map(|(s, _)| *s).collect()
    }
}

impl std::fmt::Display for GeoCube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (v, _) in &self.stickers {
            writeln!(f, "{}", v)?;
        }
        Ok(())
    }
}
