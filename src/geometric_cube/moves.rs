use cgmath::{Basis3, Deg, Rotation3};

use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::cube::GeoCube;
use super::sticker::Sticker;

pub struct GeometricMove {
    axis: Axis,
    angle: f64,
    pub predicate: Box<dyn Fn(&Sticker) -> bool>
}

impl GeometricMove {
    pub fn get_rotation_matrix(&self) -> Basis3<f64> {
        match self.axis {
            Axis::X => Basis3::from_angle_x(Deg(-self.angle)),
            Axis::Y => Basis3::from_angle_y(Deg(-self.angle)),
            Axis::Z => Basis3::from_angle_z(Deg(-self.angle))
        }
    }

    pub fn from(mv: Move, size: i32) -> Self {
        match (mv, 1) {
            (U(variant), n) | (Uw(n, variant), _) => u_move(size, n, variant),
            (R(variant), n) | (Rw(n, variant), _) => r_move(size, n, variant),
            (F(variant), n) | (Fw(n, variant), _) => f_move(size, n, variant),
            (L(variant), n) | (Lw(n, variant), _) => l_move(size, n, variant),
            (D(variant), n) | (Dw(n, variant), _) => d_move(size, n, variant),
            (B(variant), n) | (Bw(n, variant), _) => b_move(size, n, variant),
            (X(variant), _) => x_move(variant),
            (Y(variant), _) => y_move(variant),
            (Z(variant), _) => z_move(variant)
        } 
    }
}

#[derive(Copy, Clone)]
pub enum Axis {
    X, Y, Z
}

fn modify_move(mv: GeometricMove, variant: MoveVariant) -> GeometricMove {
    match variant {
        Standard => mv,
        Double => GeometricMove { angle: 2.0 * mv.angle, ..mv },
        Inverse => GeometricMove { angle: -mv.angle, ..mv }
    }
}

fn u_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove {
    modify_move(GeometricMove { 
        axis: Axis::Y, 
        angle: 90.0, 
        predicate: Box::new(move |s| s.position.y >= size - (n * 2))
    }, variant)
}
fn d_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::Y, 
        angle: -90.0, 
        predicate: Box::new(move |s| s.position.y <= -size + (n * 2))
    }, variant)
}
fn y_move(variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::Y, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) 
    }, variant)
}

fn l_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::X, 
        angle: -90.0, 
        predicate: Box::new(move |s| s.position.x <= -size + (n * 2)) 
    }, variant)
}
fn r_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove {
    modify_move(GeometricMove { 
        axis: Axis::X, 
        angle: 90.0, 
        predicate: Box::new(move |s| s.position.x >= size - (n * 2)) 
    }, variant)
}
fn x_move(variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::X, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) 
    }, variant)
}

fn f_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::Z, 
        angle: 90.0, 
        predicate: Box::new(move |s| s.position.z >= size - (n * 2))
    }, variant)
}
fn b_move(size: i32, n: i32, variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::Z, 
        angle: -90.0, 
        predicate: Box::new(move |s| s.position.z <= -size + (n * 2))
    }, variant)
}
fn z_move(variant: MoveVariant) -> GeometricMove { 
    modify_move(GeometricMove { 
        axis: Axis::Z, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) 
    }, variant) 
}