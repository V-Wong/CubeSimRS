use crate::geometric_cube::turn::{Axes, Turn};

pub static U_MOVE: Turn = Turn { axis: Axes::Y, angle: 90.0, predicate: |sticker| sticker.position.y > 0.0 };
pub static D_MOVE: Turn = Turn { axis: Axes::Y, angle: -90.0, predicate: |sticker| sticker.position.y < 0.0 };

pub static L_MOVE: Turn = Turn { axis: Axes::X, angle: 90.0, predicate: |sticker| sticker.position.x > 0.0 };
pub static R_MOVE: Turn = Turn { axis: Axes::X, angle: -90.0, predicate: |sticker| sticker.position.x < 0.0 };

pub static F_MOVE: Turn = Turn { axis: Axes::Z, angle: 90.0, predicate: |sticker| sticker.position.z > 0.0 };
pub static B_MOVE: Turn = Turn { axis: Axes::Z, angle: -90.0, predicate: |sticker| sticker.position.z < 0.0 };