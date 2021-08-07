use derive_more::Display;
use cgmath::Vector3;

pub struct Cube(Vec<Sticker>);

#[derive(Copy, Clone, Display)]
#[display(fmt = "({}, {}, {})", "position.x", "position.y", "position.z")]
struct Sticker {
    position: Vector3<i32>,
    destination: Vector3<i32>
}

impl std::fmt::Display for Cube {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for v in &self.0 {
            write!(f, "{}\n", v)?;
        }
        Ok(())
    }
}

fn create_sticker(x: i32, y: i32, z: i32) -> Sticker {
    return Sticker {
        position: Vector3 {x: x, y: y, z: z},
        destination: Vector3 {x: x, y: y, z: z}
    }
}

pub fn cube3() -> Cube {
    let mut stickers = Vec::new();

    for face in [-3, 3] {
        for p1 in [-2, 0, 2] {
            for p2 in [-2, 0, 2] {
                stickers.push(create_sticker(face, p1, p2));
                stickers.push(create_sticker(p1, face, p2));
                stickers.push(create_sticker(p1, p2, face));
            }
        }
    }

    Cube(stickers.to_vec())
}