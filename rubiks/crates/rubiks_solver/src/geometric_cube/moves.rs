use glam::DMat3;

use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;
use crate::generic_cube::{CubeSize, Move, MoveVariant};

use super::sticker::Sticker;

pub struct GeometricMove {
    axis: Axis,
    angle: f64,
    pub predicate: Box<dyn Fn(&Sticker) -> bool>,
}

impl GeometricMove {
    pub fn get_rotation_matrix(&self) -> DMat3 {
        match self.axis {
            Axis::X => DMat3::from_rotation_x(-self.angle.to_radians()),
            Axis::Y => DMat3::from_rotation_y(-self.angle.to_radians()),
            Axis::Z => DMat3::from_rotation_z(-self.angle.to_radians()),
        }
    }

    pub fn from(mv: Move) -> Self {
        match (mv, 1) {
            (U(variant), n) | (Uw(n, variant), _) => modify_move(u_move(n), variant),
            (R(variant), n) | (Rw(n, variant), _) => modify_move(r_move(n), variant),
            (F(variant), n) | (Fw(n, variant), _) => modify_move(f_move(n), variant),
            (L(variant), n) | (Lw(n, variant), _) => modify_move(l_move(n), variant),
            (D(variant), n) | (Dw(n, variant), _) => modify_move(d_move(n), variant),
            (B(variant), n) | (Bw(n, variant), _) => modify_move(b_move(n), variant),
            (X(variant), _) => modify_move(x_move(), variant),
            (Y(variant), _) => modify_move(y_move(), variant),
            (Z(variant), _) => modify_move(z_move(), variant),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

fn modify_move(mv: GeometricMove, variant: MoveVariant) -> GeometricMove {
    match variant {
        Standard => mv,
        Double => GeometricMove {
            angle: 2.0 * mv.angle,
            ..mv
        },
        Inverse => GeometricMove {
            angle: -mv.angle,
            ..mv
        },
    }
}

fn u_move(n: CubeSize) -> GeometricMove {
    GeometricMove {
        predicate: Box::new(move |s| s.current.y >= s.size - (n * 2)),
        ..y_move()
    }
}
fn d_move(n: CubeSize) -> GeometricMove {
    modify_move(
        GeometricMove {
            predicate: Box::new(move |s| s.current.y <= -s.size + (n * 2)),
            ..y_move()
        },
        Inverse,
    )
}
fn y_move() -> GeometricMove {
    GeometricMove {
        axis: Axis::Y,
        angle: 90.0,
        predicate: Box::new(|_| true),
    }
}

fn l_move(n: CubeSize) -> GeometricMove {
    modify_move(
        GeometricMove {
            predicate: Box::new(move |s| s.current.x <= -s.size + (n * 2)),
            ..x_move()
        },
        Inverse,
    )
}
fn r_move(n: CubeSize) -> GeometricMove {
    GeometricMove {
        predicate: Box::new(move |s| s.current.x >= s.size - (n * 2)),
        ..x_move()
    }
}
fn x_move() -> GeometricMove {
    GeometricMove {
        axis: Axis::X,
        angle: 90.0,
        predicate: Box::new(|_| true),
    }
}

fn f_move(n: CubeSize) -> GeometricMove {
    GeometricMove {
        predicate: Box::new(move |s| s.current.z >= s.size - (n * 2)),
        ..z_move()
    }
}
fn b_move(n: CubeSize) -> GeometricMove {
    modify_move(
        GeometricMove {
            predicate: Box::new(move |s| s.current.z <= -s.size + (n * 2)),
            ..z_move()
        },
        Inverse,
    )
}
fn z_move() -> GeometricMove {
    GeometricMove {
        axis: Axis::Z,
        angle: 90.0,
        predicate: Box::new(|_| true),
    }
}
