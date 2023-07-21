use crate::{piece::Piece, protocol::Command};
use either::{Either, Left, Right};
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Board {
    inner: HashMap<Position, Piece>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i64,
    y: i64,
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Retrieves a [`Piece`] at the passed in [`Position`] on the [`Board`].
    ///
    #[must_use]
    pub fn get(&self, position: &Position) -> Option<&Piece> {
        self.inner.get(position)
    }

    ///
    /// Places a [`Piece`] on the [`Board`] at the passed in [`Position`].
    /// Returns an [`Option`] representing:
    ///   - [`None`]: the move was invalid.
    ///   - [`Some`]: the move was valid, granting this many points.
    ///
    pub fn place(&mut self, piece: Piece, position: Position) -> bool {
        if self.inner.is_empty() {
            self.inner.insert(position, piece);
            return true;
        }

        let up = Position::new(position.x, position.y - 1);
        let down = Position::new(position.x, position.y + 1);
        let left = Position::new(position.x - 1, position.y);
        let right = Position::new(position.x + 1, position.y);

        if piece.fits(
            self.inner.get(&up),
            self.inner.get(&down),
            self.inner.get(&left),
            self.inner.get(&right),
        ) {
            self.inner.insert(position, piece);
            return true;
        }

        false
    }

    ///
    /// Removes a [`Piece`] from the [`Board`] at the passed in [`Position`].
    /// Returns an [`Option`] representing whether a [`Piece`] was actually located at the [`Position`].
    ///
    pub fn remove(&mut self, position: &Position) -> Option<Piece> {
        self.inner.remove(position)
    }

    ///
    /// Execute a [`Command`] on the [`Board`].
    ///
    #[must_use]
    pub fn execute(&mut self, command: Command) -> Either<bool, &Self> {
        match command {
            Command::Place(piece, position) => Left(self.place(piece, position)),
            Command::State => Right(&*self),
        }
    }
}

impl Position {
    pub const ORIGIN: Self = Self::new(0, 0);

    #[must_use]
    pub const fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[must_use]
    pub const fn x(&self) -> i64 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> i64 {
        self.y
    }
}

#[cfg(test)]
mod test {
    use super::{Board, Position};
    use crate::{
        piece::{Color::*, Piece, Shape::*},
        protocol::Command,
    };
    use either::Either;

    #[test]
    fn insert() {
        let mut board = Board::new();
        board.place(Piece::new(Circle, Red), Position::ORIGIN);

        assert!(board.place(Piece::new(Circle, Blue), Position::new(-1, 0)));
        assert!(board.place(Piece::new(Circle, Yellow), Position::new(1, 0)));
        assert!(board.place(Piece::new(Star8, Red), Position::new(0, 1)));
        assert!(!board.place(Piece::new(Circle, Green), Position::new(10, -15)));
    }

    #[test]
    fn execute() {
        let mut board = Board::new();
        board.place(Piece::new(Circle, Red), Position::ORIGIN);

        assert!(matches!(
            board.execute(Command::Place(
                Piece::new(Circle, Yellow),
                Position::new(1, 0),
            )),
            Either::Left(true)
        ));

        let board = board.execute(Command::State).right().unwrap();
        assert!(*board.get(&Position::ORIGIN).unwrap() == Piece::new(Circle, Red));
        assert!(*board.get(&Position::new(1, 0)).unwrap() == Piece::new(Circle, Yellow));
    }
}
