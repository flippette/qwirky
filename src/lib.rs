pub mod board;
pub mod piece;
pub mod protocol;

pub mod prelude {
    pub use crate::board::Board;
    pub use crate::piece::{
        Color::{self, *},
        Piece,
        Shape::{self, *},
    };
}
