use bevy::prelude::*;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fmt::{Display, Formatter};

/// 块
#[derive(Debug, Default, Component, Reflect, FromReflect, Clone, Copy)]
#[reflect(Component)]
pub struct Piece {
    /// 是几阶的魔方
    pub order: u8,
    /// x
    pub x: u8,
    /// y
    pub y: u8,
    /// z
    pub z: u8,
}

impl Piece {
    pub fn new(order: u8, x: u8, y: u8, z: u8) -> Self {
        Piece { order, x, y, z }
    }

    /// 判断是不是需要旋转的块
    pub fn is_selected(&self, command: &Command) -> bool {
        match command.0 {
            BaseMove::U => self.y == self.order - 1,
            BaseMove::L => self.x == 0,
            BaseMove::F => self.z == self.order - 1,
            BaseMove::R => self.x == self.order - 1,
            BaseMove::B => self.z == 0,
            BaseMove::D => self.y == 0,
            BaseMove::M => self.x == self.order / 2,
            BaseMove::E => self.y == self.order / 2,
            BaseMove::S => self.z == self.order / 2,
            BaseMove::X => true,
            BaseMove::Y => true,
            BaseMove::Z => true,
        }
    }
}

/// 表面
#[derive(Component, Reflect, FromReflect, PartialEq, Eq, Clone, Copy, Debug, Hash, Default)]
#[reflect(Component)]
pub enum Surface {
    #[default]
    U,
    D,
    L,
    R,
    F,
    B,
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
}
impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.0, self.angle().to_degrees())
    }
}

/// 是一条命令还是一组命令
#[derive(Debug, PartialEq, Eq)]
pub enum Elem {
    One(Command),
    Group(Vec<Command>, i8),
}

/// 将旋转指令打平
pub fn flatten(elems: Vec<Elem>) -> Vec<Command> {
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
#[test]
fn test_flatten() {
    let e = Elem::Group(vec![Command(BaseMove::U, 1), Command(BaseMove::R, 1)], -1);
    let f = flatten(vec![e]);
    assert_eq!(f, vec![Command(BaseMove::R, -1), Command(BaseMove::U, -1)]);
}
