use derive_more::Display;
use cgmath::{Basis3, Vector3};

pub struct Cube(Vec<Sticker>);

impl Cube {
    pub fn apply_move(&self, mv: Move) -> Cube {
        Cube(self.0.iter().map(|sticker| sticker.rotate(mv)).collect())
    }
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Display)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
struct Sticker {
    position: Vector3<f64>,
    destination: Vector3<f64>
}

impl Sticker {
    fn new(x: f64, y: f64, z: f64) -> Sticker {
        Sticker {
            position: Vector3::new(x, y, z),
            destination: Vector3::new(x, y, z)
        }
    }

    fn rotate(&self, mv: Move) -> Sticker {
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
            destination: self.destination
        }
    }
}

#[derive(Copy, Clone)]
pub struct Move {
    axis: Axes,
    angle: f64,
    predicate: fn(&Sticker) -> bool
}

#[derive(Copy, Clone)]
enum Axes {
    X, Y, Z
}

pub static U_MOVE: Move = Move { axis: Axes::Y, angle: 90.0, predicate: |sticker| sticker.position.y > 0.0 };

pub fn cube3() -> Cube {
    let mut stickers = Vec::new();

    for face in [-3.0, 3.0] {
        for p1 in [-2.0, 0.0, 2.0] {
            for p2 in [-2.0, 0.0, 2.0] {
                stickers.push(Sticker::new(face, p1, p2));
                stickers.push(Sticker::new(p1, face, p2));
                stickers.push(Sticker::new(p1, p2, face));
            }
        }
    }

    Cube(stickers.to_vec())
}