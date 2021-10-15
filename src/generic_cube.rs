/// A Rubik's Cube of arbitrary size.
/// 
/// All implementors of this trait are (externally) immutable and persistent.
/// Methods that involve mutating a Rubik's Cube will instead return a new
/// Cube with the mutation applied, leaving the old Cube intact.
pub trait Cube: Clone {
    /// Creates a solved cube of the given size.
    fn new(size: i32) -> Self;

    /// The size of the ucbe.
    fn size(&self) -> i32;

    /// A one-dimensional representation of a cube as a sequence of the faces.
    /// 
    /// # Examples
    /// 
    /// Solved 3x3x3 cube:
    /// 
    /// ```rust
    /// use cubesim::prelude::*;
    /// use cubesim::cube_implementors::FaceletCube;
    /// 
    /// /* Outputs: [U, U, U, U, U, U, U, U, U, 
    ///              R, R, R, R, R, R, R, R, R, 
    ///              F, F, F, F, F, F, F, F, F, 
    ///              D, D, D, D, D, D, D, D, D, 
    ///              L, L, L, L, L, L, L, L, L, 
    ///              B, B, B, B, B, B, B, B, B] */
    /// let cube = FaceletCube::new(3);
    /// println!("{:?}", cube.get_state());
    /// ```
    fn get_state(&self) -> Vec<Face>;

    /// Whether a cube is solved.
    fn is_solved(&self) -> bool {
        fn all_equal<T: Clone + PartialEq>(arr: &[T]) -> bool {
            arr.iter().all(|x| *x == arr[0])
        }

        let face_length = (self.size() * self.size()) as usize;
        let state = self.get_state();

        let mut is_solved = true;
        for i in 0..6 {
            let face_start = i * face_length;
            let face_end = face_start + face_length;

            is_solved = is_solved && all_equal(&state[face_start..face_end]);
        }

        is_solved
    }

    /// Creates a copy of the given cube with all faces except those in the mask
    /// replaced with a placeholder ``Face::X`` sticker.
    fn mask(&self, mask: &[i32]) -> Self;

    /// Apply a move to a cube.
    /// 
    /// # Examples
    /// 
    /// Rotate the upper layer by 90 degrees:
    /// 
    /// ```rust
    /// use cubesim::prelude::*;
    /// use cubesim::cube_implementors::FaceletCube;
    ///
    /// /* Outputs: [U, U, U, U, U, U, U, U, U, 
    ///              B, B, B, R, R, R, R, R, R, 
    ///              R, R, R, F, F, F, F, F, F, 
    ///              D, D, D, D, D, D, D, D, D, 
    ///              F, F, F, L, L, L, L, L, L, 
    ///              L, L, L, B, B, B, B, B, B] */ 
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_move(Move::U(MoveVariant::Standard));
    /// println!("{:?}", turned_cube.get_state());
    /// ```
    fn apply_move(&self, mv: Move) -> Self;

    /// Apply a sequence of moves to a cube.
    ///
    /// # Examples
    /// 
    /// Rotate the upper layer by 90 degrees:
    /// 
    /// ```rust
    /// use cubesim::prelude::*;
    /// use cubesim::cube_implementors::FaceletCube;
    ///
    /// /* Outputs: [L, L, F, U, U, D, U, U, D, 
    ///              R, R, U, R, R, U, B, B, D, 
    ///              R, R, B, F, F, B, F, F, L, 
    ///              D, D, U, D, D, U, B, R, R, 
    ///              D, F, F, D, L, L, U, L, L, 
    ///              L, B, B, L, B, B, F, F, R] */
    /// let solved_cube = FaceletCube::new(3);
    /// let turned_cube = solved_cube.apply_moves(&vec![
    ///     Move::U(MoveVariant::Standard),
    ///     Move::R(MoveVariant::Double),
    ///     Move::B(MoveVariant::Inverse),
    /// ]);
    /// println!("{:?}", turned_cube.get_state());
    /// ```
    fn apply_moves(&self, mvs: &[Move]) -> Self
    where
        Self: Sized,
    {
        let mut cube = self.clone();

        for mv in mvs {
            cube = cube.apply_move(*mv);
        }

        cube
    }
}

use derive_more::Display;

/// A face of a Rubik's Cube sticker represented in WCA notation.
///
/// The faces follow the standard WCA notation as described in the [WCA regulations].
/// 
/// [WCA regulations]: worldcubeassociation.org/regulations/#article-12-notation
#[derive(Clone, Copy, Debug, Display, Hash, PartialEq)]
pub enum Face {
    /// Upper face.
    U,
    /// Left face.
    L,
    /// Front face.
    F,
    /// Right face.
    R,
    /// Back face.
    B,
    /// Down face.
    D,
    /// Masked face. Represents a placeholder sticker.
    X,
}

/// A move of a 3 x 3 x 3 Rubik's Cube represented in WCA notation.
///
/// Each Move must be tagged with a ``MoveVariant`` to completely a move.
/// 
/// The moves follow the standard WCA notation as described in the [WCA regulations].
/// 
/// [WCA regulations]: worldcubeassociation.org/regulations/#article-12-notation
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Move {
    /// Rotate the upper layer.
    U(MoveVariant),
    /// Rotate the left layer.
    L(MoveVariant),
    /// Rotate the front layer.
    F(MoveVariant),
    /// Rotate the right layer.
    R(MoveVariant),
    /// Rotate the back layer.
    B(MoveVariant),
    /// Rotate the down layer.
    D(MoveVariant),
    Uw(i32, MoveVariant),
    Lw(i32, MoveVariant),
    Fw(i32, MoveVariant),
    Rw(i32, MoveVariant),
    Bw(i32, MoveVariant),
    Dw(i32, MoveVariant),
    /// Rotate the entire cube along the x-axis.
    X(MoveVariant),
    /// Rotate the entire cube along the y-axis.
    Y(MoveVariant),
    /// Rotate the entire cube along the z-axis.
    Z(MoveVariant),
}


/// A move variation that must be applied to the ```Move``` struct.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum MoveVariant {
    /// A 90 degree clockwise turn.
    Standard,
    /// A 180 degree clockwise turn.
    Double,
    /// A 90 degree counter-clockwise turn.
    Inverse,
}

/// A helper function to get the solved state for a cube of a given size.
pub fn solved_state(size: i32) -> Vec<Face> {
    fn repeat<T: Clone>(element: T, count: i32) -> Vec<T> {
        std::iter::repeat(element).take(count as usize).collect()
    }

    use Face::*;

    vec![repeat(U, size * size),
         repeat(R, size * size),
         repeat(F, size * size),
         repeat(D, size * size),
         repeat(L, size * size),
         repeat(B, size * size),
    ].concat()
}

/// A helper function to get all possible moves for a cube of a given size.
pub fn all_moves(size: i32) -> Vec<Move> {
    use Move::*;
    use MoveVariant::*;

    let mut moveset = Vec::new();

    for mv in [U, R, F, L, D, B] {
        for variant in [Standard, Double, Inverse] {
            moveset.push(mv(variant));
        }
    }

    for mv in [Uw, Lw, Fw, Rw, Bw, Dw] {
        for variant in [Standard, Double, Inverse] {
            for slice in 1..=(size / 2) {
                moveset.push(mv(slice, variant));
            }
        }
    }

    moveset
}
