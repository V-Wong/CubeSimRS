use derive_more::Display;
use cgmath::{Rotation, Vector3};

use crate::generic_cube::{Face, CubeSize}; 
use crate::geometric_cube::moves::{GeometricMove};

#[derive(Copy, Clone, Display, Eq, Hash, PartialEq)]
#[display(fmt = "({}, {}, {})", "initial.x", "initial.y", "initial.z")]
pub struct Sticker {
    pub size: CubeSize,
    pub current: Vector3<CubeSize>,
    pub initial: Vector3<CubeSize>,
    pub face: Face,
}

impl Sticker {
    pub fn new(size: CubeSize, x: CubeSize, y: CubeSize, z: CubeSize) -> Sticker {
        Self {
            size,
            current: Vector3::new(x, y, z),
            initial: Vector3::new(x, y, z),
            face: Self::compute_face(size, x, y, z),
        }
    }

    pub fn current_face(&self) -> Face {
        Self::compute_face(self.size, self.current.x, self.current.y, self.current.z)
    }

    pub fn initial_face(&self) -> Face {
        self.face
    }

    pub fn rotate(&self, mv: GeometricMove) -> Self {
        if !(mv.predicate)(self) {
            return *self;
        }

        let rotation_matrix = mv.get_rotation_matrix();
        let new_position = rotation_matrix.rotate_vector(
            Vector3::new(self.current.x as f64,
                         self.current.y as f64,
                         self.current.z as f64,
            )
        );

        Self {
            current: Vector3{ x: new_position.x.round() as CubeSize, 
                              y: new_position.y.round() as CubeSize, 
                              z: new_position.z.round() as CubeSize },
            ..*self
        }
    }

    pub fn compute_face(size: CubeSize, x: CubeSize, y: CubeSize, z: CubeSize) -> Face {
        if x == size { Face::R }
        else if x == -size { Face::L }
        else if y == size { Face::U }
        else if y == -size { Face::D }
        else if z == size { Face::F }
        else if z == -size { Face::B }
        else { Face::X }
    }

    pub fn set_solved(&self) -> Self {
        Self {
            current: self.initial,
            ..*self
        }
    }
}