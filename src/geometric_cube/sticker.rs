use derive_more::Display;
use cgmath::{Rotation, Vector3};

use crate::generic_cube::{Face}; 
use crate::geometric_cube::moves::{GeometricMove};

#[derive(Copy, Clone, Display, Eq, Hash, PartialEq)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
pub struct Sticker {
    pub size: i32,
    pub position: Vector3<i32>,
    pub destination: Vector3<i32>,
    pub destination_face: Face,
    pub initial_index: i32
}

impl Sticker {
    pub fn new(size: i32, x: i32, y: i32, z: i32) -> Sticker {
        Self {
            size,
            position: Vector3::new(x, y, z),
            destination: Vector3::new(x, y, z),
            destination_face: Self::get_face(size, x, y, z),
            initial_index: -1
        }
    }

    pub fn get_position_face(&self) -> Face {
        Self::get_face(self.size, self.position.x, self.position.y, self.position.z)
    }

    pub fn get_destination_face(&self) -> Face {
        self.destination_face
    }

    pub fn rotate(&self, mv: GeometricMove) -> Self {
        if !(mv.predicate)(self) {
            return *self;
        }

        let rotation_matrix = mv.get_rotation_matrix();
        let new_position = rotation_matrix.rotate_vector(
            Vector3::new(self.position.x as f64,
                         self.position.y as f64,
                         self.position.z as f64,
            )
        );

        Self {
            position: Vector3{ x: new_position.x.round() as i32, 
                               y: new_position.y.round() as i32, 
                               z: new_position.z.round() as i32 },
            ..*self
        }
    }

    pub fn get_face(size: i32, x: i32, y: i32, z: i32) -> Face {
        if x == size { Face::R }
        else if x == -size { Face::L }
        else if y == size { Face::U }
        else if y == -size { Face::D }
        else if z == size { Face::F }
        else if z == -size { Face::B }
        else { Face::X }
    }
}