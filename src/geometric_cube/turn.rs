use crate::geometric_cube::sticker::Sticker;

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