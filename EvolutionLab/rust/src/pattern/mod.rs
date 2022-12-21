pub mod rle;

pub(self) use crate::bridge::Position;

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Header {
    pub name: Option<String>,
    pub owner: Option<String>,
    pub comment: Option<String>,
    pub rule: Option<String>,
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Pattern {
    pub header: Header,
    pub cells: Vec<Position>,
}
