use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Display, Formatter};

use bevy::prelude::*;

use rubiks_solver::{Face, Move};

#[derive(Component, Debug, Clone, Copy)]
pub enum EDGE {
    UB,
    UR,
    UF,
    UL,
    FR,
    FL,
    BL,
    BR,
    DF,
    DL,
    DB,
    DR,
}

#[derive(Component, Debug, Clone, Copy)]
pub enum CORNER {
    ULB,
    URB,
    URF,
    ULF,
    DLF,
    DLB,
    DRB,
    DRF,
}

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
    /// 棱
    pub edge: Option<EDGE>,
    /// 角
    pub corner: Option<CORNER>,
}

impl Piece {
    pub fn new(size: u8, x: u8, y: u8, z: u8) -> Self {
        Piece {
            size,
            x,
            y,
            z,
            edge: None,
            corner: None,
        }
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

    pub fn is_corner(&self) -> bool {
        self.x == 0 && self.y == 0 && self.z == 0
            || self.x == 0 && self.y == 0 && self.z == self.size - 1
            || self.x == 0 && self.y == self.size - 1 && self.z == 0
            || self.x == 0 && self.y == self.size - 1 && self.z == self.size - 1
            || self.x == self.size - 1 && self.y == 0 && self.z == 0
            || self.x == self.size - 1 && self.y == 0 && self.z == self.size - 1
            || self.x == self.size - 1 && self.y == self.size - 1 && self.z == 0
            || self.x == self.size - 1 && self.y == self.size - 1 && self.z == self.size - 1
    }

    pub fn is_edge(&self) -> bool {
        self.x == 0 && self.y == 0
            || self.x == 0 && self.z == 0
            || self.y == 0 && self.z == 0
            || self.x == 0 && self.y == self.size - 1
            || self.x == 0 && self.z == self.size - 1
            || self.y == 0 && self.z == self.size - 1
            || self.x == self.size - 1 && self.y == 0
            || self.x == self.size - 1 && self.z == 0
            || self.y == self.size - 1 && self.z == 0
            || self.x == self.size - 1 && self.y == self.size - 1
            || self.x == self.size - 1 && self.z == self.size - 1
            || self.y == self.size - 1 && self.z == self.size - 1
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

/// 旋转方向
#[derive(Debug)]
pub enum Turns {
    /// 转第n个X轴多少度
    X(u8, f32),
    /// 转第n个Y轴多少度
    Y(u8, f32),
    /// 转第n个Z轴多少度
    Z(u8, f32),
}

impl Display for Turns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (n, axis, angle) = match self {
            Turns::X(n, angle) => (n, "x", angle),
            Turns::Y(n, angle) => (n, "y", angle),
            Turns::Z(n, angle) => (n, "z", angle),
        };
        write!(f, "the {} {} axis turn {}°", n, axis, angle.to_degrees())
    }
}

/// 基础的旋转指令集
/// - U: 顶面
/// - D: 底面
/// - L: 左面
/// - R: 右面
/// - F: 前面
/// - B: 后面
/// - M: L和R中间
/// - E: U和D中间
/// - S: F和B中间
/// - X: 顺时针沿着R旋转整个魔方
/// - Y: 顺时针沿着U旋转整个魔方
/// - Z: 顺时针沿着F旋转整个魔方
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum BaseMove {
    U,
    L,
    F,
    R,
    B,
    D,
    M,
    E,
    S,
    X,
    Y,
    Z,
}

impl Display for BaseMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

pub const MOVE_LIST: [BaseMove; 12] = [
    BaseMove::R,
    BaseMove::L,
    BaseMove::U,
    BaseMove::D,
    BaseMove::F,
    BaseMove::B,
    BaseMove::M,
    BaseMove::E,
    BaseMove::S,
    BaseMove::X,
    BaseMove::Y,
    BaseMove::Z,
];

/// 如何旋转面
/// - Normal 顺时针旋转
/// - Prime 逆时针旋转
/// - Double 顺时针旋转两次
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Normal,
    Prime,
    Double,
}

/// 旋转指令
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Command(pub BaseMove, pub i8);
impl Command {
    pub fn prime(self) -> Self {
        Command(self.0, -self.1)
    }

    pub fn angle(&self) -> f32 {
        match self.1 {
            1 => FRAC_PI_2,
            2 => PI,
            -1 => -FRAC_PI_2,
            -2 => -PI,
            _ => 0.0,
        }
    }

    pub fn rotation(&self) -> Quat {
        match self.0 {
            BaseMove::U => Quat::from_rotation_y(-self.angle()),
            BaseMove::L => Quat::from_rotation_x(-self.angle()),
            BaseMove::F => Quat::from_rotation_z(-self.angle()),
            BaseMove::R => Quat::from_rotation_x(-self.angle()),
            BaseMove::B => Quat::from_rotation_z(self.angle()),
            BaseMove::D => Quat::from_rotation_y(self.angle()),
            BaseMove::M => Quat::from_rotation_x(self.angle()),
            BaseMove::E => Quat::from_rotation_y(self.angle()),
            BaseMove::S => Quat::from_rotation_z(self.angle()),
            BaseMove::X => Quat::from_rotation_x(self.angle()),
            BaseMove::Y => Quat::from_rotation_y(self.angle()),
            BaseMove::Z => Quat::from_rotation_z(self.angle()),
        }
    }

    pub fn axis(&self) -> Vec3 {
        match self.0 {
            BaseMove::U => Vec3::Y,
            BaseMove::L => Vec3::X,
            BaseMove::F => Vec3::Z,
            BaseMove::R => Vec3::X,
            BaseMove::B => Vec3::Z,
            BaseMove::D => Vec3::Y,
            BaseMove::M => Vec3::X,
            BaseMove::E => Vec3::Y,
            BaseMove::S => Vec3::Z,
            BaseMove::X => Vec3::X,
            BaseMove::Y => Vec3::Y,
            BaseMove::Z => Vec3::Z,
        }
    }

    pub fn clockwise(&self) -> bool {
        self.1 > 0
    }
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.angle().to_degrees())
    }
}

/// 是一条命令还是一组命令
#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    One(Move),
    Group(Vec<Move>, i8),
}

/// 将旋转指令打平
pub fn flatten(elems: Vec<Elem>) -> Vec<Move> {
    let mut v = vec![];
    for e in elems {
        match e {
            Elem::One(c) => v.push(c),
            Elem::Group(cs, rep) => {
                if rep > 0 {
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c);
                        }
                    }
                } else {
                    let rep = -rep;
                    let mut cs = cs;
                    cs.reverse();
                    for _ in 0..rep {
                        for &c in &cs {
                            v.push(c.prime())
                        }
                    }
                }
            }
        }
    }
    v
}

pub struct MyRaycastSet;
