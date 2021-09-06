/// A Rubik's Cube of arbitrary size.
/// 
/// All implementors of this trait are (externally) immutable and persistent.
/// Methods that involve mutating a Rubik's Cube will instead return a new
/// Cube with the mutation applied, leaving the old Cube intact.
pub trait Cube: Clone {
    /// Creates a solved cube of the given size.
    fn new(size: i32) -> Self;

    /// Creates a solved cube of the given size,
    /// with all faces except those in the mask
    /// replaced with a placeholder ``Face::X`` sticker.
    fn mask(size: i32, mask: &[i32]) -> Self;

    /// Whether a cube is solved.
    fn is_solved(&self) -> bool;

    /// A one-dimensional representation of a cube
    /// as a sequence of the faces.
    /// 
    /// # Examples
    /// 
    /// Solved 3x3x3 cube:
    /// 
    /// ```rust
    /// use cubesim::{Cube, FaceletCube};
    /// let cube = FaceletCube::new(3);
    /// 
    /// /* Outputs: [U, U, U, U, U, U, U, U, U, 
    ///              R, R, R, R, R, R, R, R, R, 
    ///              F, F, F, F, F, F, F, F, F, 
    ///              D, D, D, D, D, D, D, D, D, 
    ///              L, L, L, L, L, L, L, L, L, 
    ///              B, B, B, B, B, B, B, B, B] */
    /// println!("{:?}", cube.get_state());
    /// ```
    fn get_state(&self) -> Vec<Face>;

    /// Apply a move to a cube.
    /// 
    /// # Examples
    /// 
    /// Rotate the upper layer by 90 degrees:
    /// 
    /// ```rust
    /// use cubesim::{Cube, Move, MoveVariant, FaceletCube};
    /// let cube = FaceletCube::new(3);
    ///
    /// let turned_cube = cube.apply_move(Move::U(MoveVariant::Standard));
    /// ```
    fn apply_move(&self, mv: Move) -> Self;

    /// Apply a sequence of moves to a cube.
    ///
    /// # Examples
    /// 
    /// Rotate the upper layer by 90 degrees:
    /// 
    /// ```rust
    /// use cubesim::{Cube, Move, MoveVariant, FaceletCube};
    /// let cube = FaceletCube::new(3);
    ///
    /// let turned_cube = cube.apply_move(vec![
    ///     Move::U(MoveVariant::Standard),
    ///     Move::R(MoveVariant::Double),
    ///     Move::B(MoveVariant::Inverse),
    /// ]);
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

/// A face of a Rubik's Cube sticker represented in WCA notation.
///
/// The faces follow the standard WCA notation as described in
/// [WCA regulations](worldcubeassociation.org/regulations/#article-12-notation).
#[derive(Clone, Copy, Debug, Hash, PartialEq)]
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
/// Each Move must be tagged with a ``MoveVariant``
/// to completey define the type of move.
/// 
/// The moves follow the standard WCA notation as described in the
/// [WCA regulations](worldcubeassociation.org/regulations/#article-12-notation).
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