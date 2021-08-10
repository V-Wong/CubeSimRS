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

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Move {
    U, L, F, R, B, D, X, Y, Z,
    U_, L_, F_, R_, B_, D_, X_, Y_, Z_,
    U2, L2, F2, R2, B2, D2, X2, Y2, Z2,
}