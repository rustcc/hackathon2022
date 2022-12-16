use std::sync::Mutex;

use cached::proc_macro::cached;
use glam::IVec3;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::generic_cube::{Cube, CubeSize, Move};
use crate::geometric_cube::{GeoCube, Sticker};

pub struct FaceletMove(pub Vec<(u16, u16)>);

pub fn compute_permutation<T: Clone + Copy>(old_faces: &[T], size: CubeSize, mv: Move) -> Vec<T> {
    lazy_static! {
        static ref CACHE: Mutex<FxHashMap<(CubeSize, Move), FaceletMove>> =
            Mutex::new(FxHashMap::default());
    }

    let mut cache = CACHE.lock().unwrap();

    let facelet_move = if let Some(res) = cache.get(&(size, mv)) {
        res
    } else {
        cache.insert((size, mv), convert_move(size, mv));
        cache.get(&(size, mv)).unwrap()
    };

    let mut new_faces = old_faces.to_owned();

    for (x, y) in &facelet_move.0 {
        new_faces[*y as usize] = old_faces[*x as usize];
    }

    new_faces
}

fn convert_move(size: CubeSize, mv: Move) -> FaceletMove {
    let index_map = create_piece_map(size);

    FaceletMove(
        GeoCube::new(size)
            .apply_move(mv)
            .stickers()
            .iter()
            .map(|s| {
                (
                    index_map[(&s.initial)] as u16,
                    index_map[(&s.current)] as u16,
                )
            })
            .filter(|x| x.0 != x.1)
            .collect(),
    )
}

#[cached]
fn create_piece_map(size: CubeSize) -> FxHashMap<IVec3, u16> {
    let mut map = FxHashMap::default();

    let face_rotating_moves = vec![
        vec![],
        vec![X(Inverse), Y(Inverse)],
        vec![X(Inverse)],
        vec![X(Double)],
        vec![X(Inverse), Y(Standard)],
        vec![X(Inverse), Y(Double)],
    ];

    let mut idx = 0;
    for rotation in face_rotating_moves {
        for z in GeoCube::range(size) {
            for x in GeoCube::range(size) {
                let first_sticker = GeoCube {
                    size,
                    stickers: vec![(Sticker::new(size, x, size, z), 0)],
                }
                .apply_moves(&rotation)
                .stickers()[0];
                map.insert(first_sticker.current, idx);
                idx += 1;
            }
        }
    }

    map
}
