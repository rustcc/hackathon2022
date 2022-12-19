use bevy::prelude::*;

use rubiks_solver::{Face, Move};

/// 块
#[derive(Debug, Default, Component, Clone, Copy)]
pub struct Piece {
    /// 是几阶的魔方
    pub size: u8,
    /// x
    pub x: u8,
    /// y
    pub y: u8,
    /// z
    pub z: u8,
}

impl Piece {
    pub fn new(size: u8, x: u8, y: u8, z: u8) -> Self {
        Piece { size, x, y, z }
    }

    /// 判断是不是需要旋转的块
    pub fn is_selected(&self, command: &Move) -> bool {
        match command {
            Move::U(_) => self.y == self.size - 1,
            Move::L(_) => self.x == 0,
            Move::F(_) => self.z == self.size - 1,
            Move::R(_) => self.x == self.size - 1,
            Move::B(_) => self.z == 0,
            Move::D(_) => self.y == 0,
            Move::X(_) => true,
            Move::Y(_) => true,
            Move::Z(_) => true,
            Move::M(_) => false,
            Move::E(_) => false,
            Move::S(_) => false,
            Move::Uw(n, _) => self.y == self.size - *n as u8,
            Move::Lw(n, _) => self.x == *n as u8 - 1,
            Move::Fw(n, _) => self.z == self.size - *n as u8,
            Move::Rw(n, _) => self.x == self.size - *n as u8,
            Move::Bw(n, _) => self.z == *n as u8 - 1,
            Move::Dw(n, _) => self.y == *n as u8 - 1,
        }
    }

    /// 检查块的颜色
    pub fn has_face(&self, face: Face) -> bool {
        match face {
            Face::U => self.y == self.size - 1,
            Face::L => self.x == 0,
            Face::F => self.z == self.size - 1,
            Face::R => self.x == self.size - 1,
            Face::B => self.z == 0,
            Face::D => self.y == 0,
            _ => false,
        }
    }

    pub fn is_up(&self) -> bool {
        self.y == self.size - 1
    }

    pub fn get_up_n(&self) -> u8 {
        self.size - self.y
    }

    pub fn is_down(&self) -> bool {
        self.y == 0
    }

    pub fn get_down_n(&self) -> u8 {
        self.y + 1
    }

    pub fn is_front(&self) -> bool {
        self.z == self.size - 1
    }

    pub fn get_front_n(&self) -> u8 {
        self.size - self.z
    }

    pub fn is_back(&self) -> bool {
        self.z == 0
    }

    pub fn get_back_n(&self) -> u8 {
        self.z + 1
    }

    pub fn is_left(&self) -> bool {
        self.x == 0
    }

    pub fn get_left_n(&self) -> u8 {
        self.x + 1
    }

    pub fn is_right(&self) -> bool {
        self.x == self.size - 1
    }

    pub fn get_right_n(&self) -> u8 {
        self.size - self.x
    }

    pub fn is_in_m_layer(&self) -> bool {
        !self.is_left() && !self.is_right()
    }

    pub fn is_in_e_layer(&self) -> bool {
        !self.is_up() && !self.is_down()
    }

    pub fn is_in_s_layer(&self) -> bool {
        !self.is_front() && !self.is_back()
    }
}

/// 表面
#[derive(Component, PartialEq, Eq, Clone, Copy, Debug, Hash)]
pub struct Surface {
    pub initial: Face,
}

pub struct MyRaycastSet;
