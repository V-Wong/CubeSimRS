use cgmath::{Basis3, Deg, Rotation3};

use crate::generic_cube::{Move};

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
            Axes::X => Basis3::from_angle_x(Deg(self.angle)),
            Axes::Y => Basis3::from_angle_y(Deg(self.angle)),
            Axes::Z => Basis3::from_angle_z(Deg(self.angle))
        }
    }
}

#[derive(Copy, Clone)]
pub enum Axes {
    X, Y, Z
}

pub fn convert_move(mv: Move) -> Turn {
    match mv {
        Move::U => U_MOVE,
        Move::R => R_MOVE,
        Move::F => F_MOVE,
        Move::L => L_MOVE,
        Move::D => D_MOVE,
        Move::B => B_MOVE,
        Move::X => X_MOVE,
        Move::Y => Y_MOVE,
        Move::Z => Z_MOVE,
        Move::U_ => invert_move(U_MOVE),
        Move::R_ => invert_move(R_MOVE),
        Move::F_ => invert_move(F_MOVE),
        Move::L_ => invert_move(L_MOVE),
        Move::D_ => invert_move(D_MOVE),
        Move::B_ => invert_move(B_MOVE),
        Move::X_ => invert_move(X_MOVE),
        Move::Y_ => invert_move(Y_MOVE),
        Move::Z_ => invert_move(Z_MOVE),
        Move::U2 => double_move(U_MOVE),
        Move::R2 => double_move(R_MOVE),
        Move::F2 => double_move(F_MOVE),
        Move::L2 => double_move(L_MOVE),
        Move::D2 => double_move(D_MOVE),
        Move::B2 => double_move(B_MOVE),
        Move::X2 => double_move(X_MOVE),
        Move::Y2 => double_move(Y_MOVE),
        Move::Z2 => double_move(Z_MOVE)
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

pub static L_MOVE: Turn = Turn { axis: Axes::X, angle: -90.0, predicate: |sticker| sticker.position.x > 0.0 };
pub static R_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |sticker| sticker.position.x < 0.0 };
pub static X_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |_| true };

pub static F_MOVE: Turn = Turn { axis: Axes::Z, angle: -90.0, predicate: |sticker| sticker.position.z > 0.0 };
pub static B_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |sticker| sticker.position.z < 0.0 };
pub static Z_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |_| true };
