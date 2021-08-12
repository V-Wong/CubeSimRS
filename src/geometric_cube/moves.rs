use cgmath::{Basis3, Deg, Rotation3};

use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::sticker::Sticker;

#[derive(Copy, Clone)]
pub struct Turn {
    axis: Axes,
    angle: f64,
    pub predicate: fn(&Sticker) -> bool
}

impl Turn {
    pub fn get_rotation_matrix(&self) -> Basis3<f64> {
        match self.axis {
            Axes::X => Basis3::from_angle_x(Deg(-self.angle)),
            Axes::Y => Basis3::from_angle_y(Deg(-self.angle)),
            Axes::Z => Basis3::from_angle_z(Deg(-self.angle))
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axes {
    X, Y, Z
}

pub fn convert_move(mv: Move) -> Turn {
    match mv {
        U(variant) => modify_move(U_MOVE, variant),
        R(variant) => modify_move(R_MOVE, variant),
        F(variant) => modify_move(F_MOVE, variant),
        L(variant) => modify_move(L_MOVE, variant),
        D(variant) => modify_move(D_MOVE, variant),
        B(variant) => modify_move(B_MOVE, variant),
        X(variant) => modify_move(X_MOVE, variant),
        Y(variant) => modify_move(Y_MOVE, variant),
        Z(variant) => modify_move(Z_MOVE, variant)
    }
}

pub fn modify_move(mv: Turn, variant: MoveVariant) -> Turn {
    match variant {
        Standard => mv,
        Double => double_move(mv),
        Inverse => invert_move(mv)
    }
}

pub fn invert_move(mv: Turn) -> Turn {
    Turn {
        angle: -mv.angle,
        ..mv
    }
}

pub fn double_move(mv: Turn) -> Turn {
    Turn {
        angle: 2.0 * mv.angle,
        ..mv
    }
}

pub static U_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |sticker| sticker.position.y > 0.0 };
pub static D_MOVE: Turn = Turn { axis: Axes::Y, angle: -90.0, predicate: |sticker| sticker.position.y < 0.0 };
pub static Y_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |_| true };

pub static L_MOVE: Turn = Turn { axis: Axes::X, angle: -90.0, predicate: |sticker| sticker.position.x < 0.0 };
pub static R_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |sticker| sticker.position.x > 0.0 };
pub static X_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |_| true };

pub static F_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |sticker| sticker.position.z > 0.0 };
pub static B_MOVE: Turn = Turn { axis: Axes::Z, angle: -90.0, predicate: |sticker| sticker.position.z < 0.0 };
pub static Z_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |_| true };
