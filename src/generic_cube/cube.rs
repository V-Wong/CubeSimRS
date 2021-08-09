pub trait Cube {
    fn is_solved(&self) -> bool;
    fn get_state(&self) -> Vec<Face>;
    fn apply_move(&self, mv: Move) -> Box<dyn Cube>;
}

#[derive(Debug)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
    X
}

#[derive(Debug)]
pub enum Move {
    U,
    L,
    F,
    R,
    B,
    D,
}