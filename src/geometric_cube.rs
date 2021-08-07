use derive_more::Display;
use cgmath::Vector3;

pub struct Cube(Vec<Sticker>);

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
    position: Vector3<i32>,
    destination: Vector3<i32>
}

impl Sticker {
    fn new(x: i32, y: i32, z: i32) -> Sticker {
        Sticker {
            position: Vector3::new(x, y, z),
            destination: Vector3::new(x, y, z)
        }
    }
}

pub fn cube3() -> Cube {
    let mut stickers = Vec::new();

    for face in [-3, 3] {
        for p1 in [-2, 0, 2] {
            for p2 in [-2, 0, 2] {
                stickers.push(Sticker::new(face, p1, p2));
                stickers.push(Sticker::new(p1, face, p2));
                stickers.push(Sticker::new(p1, p2, face));
            }
        }
    }

    Cube(stickers.to_vec())
}
