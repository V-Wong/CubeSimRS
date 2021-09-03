pub trait Cube: Clone {
    fn new(size: i32) -> Self;
    fn mask(size: i32, mask: &Vec<i32>) -> Self;

    fn is_solved(&self) -> bool;
    fn get_state(&self) -> Vec<Face>;
    fn apply_move(&self, mv: Move) -> Self;

    fn apply_moves(&self, mvs: Vec<Move>) -> Self where Self: Sized {
        let mut cube = self.clone();

        for mv in mvs {
            cube = cube.apply_move(mv);
        }

        cube
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
pub enum Face {
    U, L, F, R, B, D, X
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    U(MoveVariant), 
    L(MoveVariant), 
    F(MoveVariant), 
    R(MoveVariant), 
    B(MoveVariant), 
    D(MoveVariant), 
    X(MoveVariant), 
    Y(MoveVariant), 
    Z(MoveVariant)
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveVariant {
    Standard,
    Double,
    Inverse,
}