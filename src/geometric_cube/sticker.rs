use derive_more::Display;
use cgmath::{Basis3, Vector3};

use crate::geometric_cube::turn::{Turn, Axes};

#[derive(Copy, Clone, Display)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
pub struct Sticker {
    pub position: Vector3<f64>,
    pub destination: Vector3<f64>
}

#[derive(Debug)]
pub enum Faces {
    U,
    L,
    F,
    R,
    B,
    D,
    X
}

impl Sticker {
    pub fn new(x: f64, y: f64, z: f64) -> Sticker {
        Sticker {
            position: Vector3::new(x, y, z),
            destination: Vector3::new(x, y, z)
        }
    }

    pub fn is_solved(&self) -> bool {
        self.position == self.destination
    }

    pub fn get_position_face(&self) -> Faces {
        Self::get_face(self.position.x, self.position.y, self.position.z)
    }

    pub fn get_destination_face(&self) -> Faces {
        Self::get_face(self.destination.x, self.destination.y, self.destination.z)
    }

    pub fn rotate(&self, mv: Turn) -> Sticker {
        if !(mv.predicate)(self) {
            return *self;
        }

        use cgmath::{Deg, Rotation, Rotation3};

        let rotation_matrix = {
            match mv.axis {
                Axes::X => Basis3::from_angle_x(Deg(mv.angle)),
                Axes::Y => Basis3::from_angle_y(Deg(mv.angle)),
                Axes::Z => Basis3::from_angle_z(Deg(mv.angle))
            }
        };

        let new_position = rotation_matrix.rotate_vector(self.position);

        Sticker {
            position: Vector3{ x: new_position.x.round(), 
                               y: new_position.y.round(), 
                               z: new_position.z.round() },
            ..*self
        }
    }

    fn get_face(x: f64, y: f64, z: f64) -> Faces {
        match (x, y, z) {
            (3.0, _, _) => Faces::R,
            (-3.0, _, _) => Faces::L,
            (_, 3.0, _) => Faces::U,
            (_, -3.0, _) => Faces::D,
            (_, _, -3.0) => Faces::F,
            (_, _, 3.0) => Faces::B,
            (_, _, _) => Faces::X
        }
    }
}