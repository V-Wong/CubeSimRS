use cgmath::{Basis3, Deg, Rotation3};

use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

use super::cube::GeoCube;
use super::sticker::Sticker;

pub struct GeometricMove {
    axis: Axes,
    angle: f64,
    pub predicate: Box<dyn Fn(&Sticker) -> bool>
}

impl GeometricMove {
    pub fn get_rotation_matrix(&self) -> Basis3<f64> {
        match self.axis {
            Axes::X => Basis3::from_angle_x(Deg(-self.angle)),
            Axes::Y => Basis3::from_angle_y(Deg(-self.angle)),
            Axes::Z => Basis3::from_angle_z(Deg(-self.angle))
        }
    }

    pub fn from(mv: Move, size: i32) -> Self {
        let slices = GeoCube::range(size).into_iter();

        match (mv, 1) {
            (U(variant), n) | (Uw(n, variant), _) => modify_move(u_move(size, slices.rev().take(n as usize).collect()), variant),
            (R(variant), n) | (Rw(n, variant), _) => modify_move(r_move(size, slices.rev().take(n as usize).collect()), variant),
            (F(variant), n) | (Fw(n, variant), _) => modify_move(f_move(size, slices.rev().take(n as usize).collect()), variant),
            (L(variant), n) | (Lw(n, variant), _) => modify_move(l_move(size, slices.take(n as usize).collect()), variant),
            (D(variant), n) | (Dw(n, variant), _) => modify_move(d_move(size, slices.take(n as usize).collect()), variant),
            (B(variant), n) | (Bw(n, variant), _) => modify_move(b_move(size, slices.take(n as usize).collect()), variant),
            (X(variant), _) => modify_move(x_move(), variant),
            (Y(variant), _) => modify_move(y_move(), variant),
            (Z(variant), _) => modify_move(z_move(), variant)
        } 
    }
}

#[derive(Copy, Clone)]
pub enum Axes {
    X, Y, Z
}

pub fn modify_move(mv: GeometricMove, variant: MoveVariant) -> GeometricMove {
    match variant {
        Standard => mv,
        Double => GeometricMove { angle: 2.0 * mv.angle, ..mv },
        Inverse => GeometricMove { angle: -mv.angle, ..mv }
    }
}

fn u_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Y, 
        angle: 90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.y) || s.position.y == size) 
    } 
}
fn d_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Y, 
        angle: -90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.y) || s.position.y == -size)
    } 
}
fn y_move() -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Y, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) 
    } 
}

fn l_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::X, 
        angle: -90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.x) || s.position.x == -size) 
    } 
}
fn r_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::X, 
        angle: 90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.x) || s.position.x == size) 
    } 
}
fn x_move() -> GeometricMove { 
    GeometricMove { 
        axis: Axes::X, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) 
    } 
}

fn f_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Z, 
        angle: 90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.z) || s.position.z == size)
    } 
}

fn b_move(size: i32, slices: Vec<i32>) -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Z, 
        angle: -90.0, 
        predicate: Box::new(move |s| slices.contains(&s.position.z) || s.position.z == -size) 
    } 
}

fn z_move() -> GeometricMove { 
    GeometricMove { 
        axis: Axes::Z, 
        angle: 90.0, 
        predicate: Box::new(move |_| true) } 
}