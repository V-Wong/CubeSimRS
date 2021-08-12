use derive_more::Display;
use cgmath::{Rotation, Vector3};

use crate::generic_cube::{Face}; 
use crate::geometric_cube::moves::{Turn};

#[derive(Copy, Clone, Display)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
pub struct Sticker {
    pub position: Vector3<f64>,
    pub destination: Vector3<f64>
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

    pub fn get_position_face(&self) -> Face {
        Self::get_face(self.position.x, self.position.y, self.position.z)
    }

    pub fn get_destination_face(&self) -> Face {
        Self::get_face(self.destination.x, self.destination.y, self.destination.z)
    }

    pub fn rotate(&self, mv: Turn) -> Sticker {
        if !(mv.predicate)(self) {
            return *self;
        }

        let rotation_matrix = mv.get_rotation_matrix();
        let new_position = rotation_matrix.rotate_vector(self.position);

        Sticker {
            position: Vector3{ x: new_position.x.round(), 
                               y: new_position.y.round(), 
                               z: new_position.z.round() },
            ..*self
        }
    }

    fn get_face(x: f64, y: f64, z: f64) -> Face {
        match (x as i64, y as i64, z as i64) {
            (3, _, _) => Face::R,
            (-3, _, _) => Face::L,
            (_, 3, _) => Face::U,
            (_, -3, _) => Face::D,
            (_, _, 3) => Face::F,
            (_, _, -3) => Face::B,
            (_, _, _) => Face::X
        }
    }
}