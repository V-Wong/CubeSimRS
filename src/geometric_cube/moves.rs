use super::sticker::Sticker;

#[derive(Copy, Clone)]
pub struct Turn {
    pub axis: Axes,
    pub angle: f64,
    pub predicate: fn(&Sticker) -> bool
}

#[derive(Copy, Clone)]
pub enum Axes {
    X, Y, Z
}

pub fn invert(mv: Turn) -> Turn {
    Turn {
        angle: -mv.angle,
        ..mv
    }
}

pub static U_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |sticker| sticker.position.y > 0.0 };
pub static D_MOVE: Turn = Turn { axis: Axes::Y, angle: -90.0, predicate: |sticker| sticker.position.y < 0.0 };
pub static Y_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |_| true };

pub static L_MOVE: Turn = Turn { axis: Axes::X, angle: -90.0, predicate: |sticker| sticker.position.x > 0.0 };
pub static R_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |sticker| sticker.position.x < 0.0 };
pub static X_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |_| true };

pub static F_MOVE: Turn = Turn { axis: Axes::Z, angle: -90.0, predicate: |sticker| sticker.position.z > 0.0 };
pub static B_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |sticker| sticker.position.z < 0.0 };
pub static Z_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |_| true };
