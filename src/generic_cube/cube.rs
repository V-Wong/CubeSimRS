pub trait Cube: Clone {
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

#[derive(Clone, Copy, Debug)]
pub enum Face {
    U,
    L,
    F,
    R,
    B,
    D,
    X
}

#[derive(Clone, Copy, Debug)]
pub enum Move {
    U,
    L,
    F,
    R,
    B,
    D,
    X,
    Y,
    Z
}