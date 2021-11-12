use derive_more::Display;
use cgmath::{Rotation, Vector3};

use crate::generic_cube::{Face, CubeSize}; 
use crate::geometric_cube::moves::{GeometricMove};

#[derive(Copy, Clone, Display, Eq, Hash, PartialEq)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
pub struct Sticker {
    pub size: CubeSize,
    pub position: Vector3<CubeSize>,
    pub destination: Vector3<CubeSize>,
    pub destination_face: Face,
}

impl Sticker {
    pub fn new(size: CubeSize, x: CubeSize, y: CubeSize, z: CubeSize) -> Sticker {
        Self {
            size,
            position: Vector3::new(x, y, z),
            destination: Vector3::new(x, y, z),
            destination_face: Self::face(size, x, y, z),
        }
    }

    pub fn position_face(&self) -> Face {
        Self::face(self.size, self.position.x, self.position.y, self.position.z)
    }

    pub fn destination_face(&self) -> Face {
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
            position: Vector3{ x: new_position.x.round() as CubeSize, 
                               y: new_position.y.round() as CubeSize, 
                               z: new_position.z.round() as CubeSize },
            ..*self
        }
    }

    pub fn face(size: CubeSize, x: CubeSize, y: CubeSize, z: CubeSize) -> Face {
        if x == size { Face::R }
        else if x == -size { Face::L }
        else if y == size { Face::U }
        else if y == -size { Face::D }
        else if z == size { Face::F }
        else if z == -size { Face::B }
        else { Face::X }
    }
}