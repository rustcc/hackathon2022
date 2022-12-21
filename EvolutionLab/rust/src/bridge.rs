pub use std::sync::Mutex;
use std::sync::MutexGuard;

use anyhow::Result;
use flutter_rust_bridge::RustOpaque;

pub use crate::array_life::ArrayLife;
pub use crate::pattern::{Header, Pattern};

/// 边界条件
/// Sphere 循环;
/// Mirror 镜像;
/// None 截断;
#[derive(Debug, Eq, PartialEq)]
pub enum Boundary {
    Sphere,
    Mirror,
    None,
}

// 网格形状(大小)
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Shape {
    pub x: usize,
    pub y: usize,
}

// 细胞位置
#[derive(Debug, Eq, PartialEq, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub trait LifeAPI {
    fn evolve(&mut self, step: Option<u32>);

    fn clean_cells(&mut self);

    fn rand(&mut self, distr: f64);

    fn get_cells(&self) -> Vec<Position>;

    fn set_cells(&mut self, cells: Vec<Position>);

    fn set_boundary(&mut self, boundary: Boundary);

    fn set_shape(&mut self, shape: Shape, clean: Option<bool>);
}

// flutter_rust_bridge 要求 &self，使用 Mutex 修改内部结构
pub struct Life(pub RustOpaque<Mutex<ArrayLife>>);

impl Life {
    fn lock(&self) -> MutexGuard<ArrayLife> {
        self.0.lock().unwrap()
    }

    pub fn evolve(&self, step: Option<u32>) {
        self.lock().evolve(step);
    }

    pub fn clean_cells(&self) {
        self.lock().clean_cells();
    }

    pub fn rand(&self, distr: f64) {
        self.lock().rand(distr);
    }

    pub fn get_cells(&self) -> Vec<Position> {
        self.lock().get_cells()
    }

    pub fn set_cells(&self, cells: Vec<Position>) {
        self.lock().set_cells(cells);
    }

    pub fn set_boundary(&self, boundary: Boundary) {
        self.lock().set_boundary(boundary);
    }

    pub fn set_shape(&self, shape: Shape, clean: Option<bool>) {
        self.lock().set_shape(shape, clean);
    }
}

pub fn create(shape: Shape, boundary: Boundary) -> Life {
    let array_life = ArrayLife::new(shape, boundary);

    Life(RustOpaque::new(Mutex::new(array_life)))
}

pub fn decode_rle(rle: String) -> Result<Pattern> {
    Pattern::decode_rle(&rle[..])
}

pub fn encode_rle(header: Header, cells: Vec<Position>) -> String {
    Pattern { header, cells }.encode_rle()
}

pub fn default_pattern() -> Pattern {
    include!("pattern/gospers_glider_gun_synth.txt")
}
