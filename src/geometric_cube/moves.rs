use cgmath::{Basis3, Deg, Rotation3};

use crate::generic_cube::{Move, MoveVariant};
use crate::generic_cube::Move::*;
use crate::generic_cube::MoveVariant::*;

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

    pub fn from(mv: Move) -> Self {
        let gmove = match (mv, 1) {
            (U(variant), n) | (Uw(n, variant), _) => modify_move(u_move(n), variant),
            (R(variant), n) | (Rw(n, variant), _) => modify_move(r_move(n), variant),
            (F(variant), n) | (Fw(n, variant), _) => modify_move(f_move(n), variant),
            (L(variant), n) | (Lw(n, variant), _) => modify_move(l_move(n), variant),
            (D(variant), n) | (Dw(n, variant), _) => modify_move(d_move(n), variant),
            (B(variant), n) | (Bw(n, variant), _) => modify_move(b_move(n), variant),
            (X(variant), _) => modify_move(x_move(), variant),
            (Y(variant), _) => modify_move(y_move(), variant),
            (Z(variant), _) => modify_move(z_move(), variant),
        };
        gmove
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

fn u_move(n: i32) -> GeometricMove {
    GeometricMove { 
        predicate: Box::new(move |s| s.position.y >= s.size - (n * 2)),
        ..y_move()
    }
}
fn d_move(n: i32) -> GeometricMove { 
    modify_move(GeometricMove { 
        predicate: Box::new(move |s| s.position.y <= -s.size + (n * 2)),
        ..y_move()
    }, Inverse)
}
fn y_move() -> GeometricMove { 
    GeometricMove { axis: Axis::Y, angle: 90.0, predicate: Box::new(|_| true) }
}

fn l_move(n: i32) -> GeometricMove { 
    modify_move(GeometricMove { 
        predicate: Box::new(move |s| s.position.x <= -s.size + (n * 2)),
        ..x_move() 
    }, Inverse)
}
fn r_move(n: i32) -> GeometricMove {
    GeometricMove { 
        predicate: Box::new(move |s| s.position.x >= s.size - (n * 2)),
        ..x_move()
    }
}
fn x_move() -> GeometricMove { 
    GeometricMove { axis: Axis::X, angle: 90.0, predicate: Box::new(|_| true) }
}

fn f_move(n: i32) -> GeometricMove { 
    GeometricMove { 
        predicate: Box::new(move |s| s.position.z >= s.size - (n * 2)),
        ..z_move()
    }
}
fn b_move(n: i32) -> GeometricMove { 
    modify_move(GeometricMove { 
        predicate: Box::new(move |s| s.position.z <= -s.size + (n * 2)),
        ..z_move()
    }, Inverse)
}
fn z_move() -> GeometricMove { 
    GeometricMove { axis: Axis::Z, angle: 90.0, predicate: Box::new(|_| true) }
}