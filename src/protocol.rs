use crate::{
    board::Position,
    piece::{
        Color::{self, *},
        Piece,
        Shape::{self, *},
    },
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64,
    combinator::{map_res, opt},
    error::Error,
    sequence::tuple,
    Err,
};

type Input<'a> = &'a str;
type Output<'a, O> = nom::IResult<Input<'a>, O, Error<Input<'a>>>;

///
/// Command passed in by the user interface.
/// Syntax: `<type>:<args>`
///
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Place(Piece, Position),
    State,
}

impl Command {
    pub fn parse(s: &str) -> Result<Command, Err<Error<&str>>> {
        let (s, (command_type, _)) = tuple((command_type, opt(tag(":"))))(s)?;
        match command_type {
            "place" => {
                let (piece, position) = place_args(s)?.1;
                Ok(Command::Place(piece, position))
            }
            "state" => Ok(Command::State),
            _ => unreachable!(),
        }
    }
}

fn command_type(i: Input) -> Output<&str> {
    alt((tag("place"), tag("state")))(i)
}

fn place_args(i: Input) -> Output<(Piece, Position)> {
    let (i, (piece, _, position)) = tuple((piece, tag("@"), position))(i)?;

    Ok((i, (piece, position)))
}

fn piece(i: Input) -> Output<Piece> {
    let (i, (shape, _, color)) = tuple((shape, tag("&"), color))(i)?;

    Ok((i, Piece::new(shape, color)))
}

fn position(i: Input) -> Output<Position> {
    let (i, (x, _, y)) = tuple((i64, tag(","), i64))(i)?;

    Ok((i, Position::new(x, y)))
}

fn shape(i: Input) -> Output<Shape> {
    type E<'a> = Err<Error<&'a str>>;

    let (i, shape) = alt((
        map_res(tag("circle"), |_| Ok::<_, E>(Circle)),
        map_res(tag("star4"), |_| Ok::<_, E>(Star4)),
        map_res(tag("diamond"), |_| Ok::<_, E>(Diamond)),
        map_res(tag("square"), |_| Ok::<_, E>(Square)),
        map_res(tag("star8"), |_| Ok::<_, E>(Star8)),
        map_res(tag("clover"), |_| Ok::<_, E>(Clover)),
    ))(i)?;

    Ok((i, shape))
}

fn color(i: Input) -> Output<Color> {
    type E<'a> = Err<Error<&'a str>>;

    let (i, color) = alt((
        map_res(tag("red"), |_| Ok::<_, E>(Red)),
        map_res(tag("orange"), |_| Ok::<_, E>(Orange)),
        map_res(tag("yellow"), |_| Ok::<_, E>(Yellow)),
        map_res(tag("green"), |_| Ok::<_, E>(Green)),
        map_res(tag("blue"), |_| Ok::<_, E>(Blue)),
        map_res(tag("purple"), |_| Ok::<_, E>(Purple)),
    ))(i)?;

    Ok((i, color))
}

#[cfg(test)]
mod test {
    use super::Command;
    use crate::{
        board::Position,
        piece::{Color::*, Piece, Shape::*},
    };
    use nom::{error::ErrorKind, Err};

    #[test]
    fn place() {
        assert!(matches!(
            Command::parse("place:circle&red@0,1"),
            Ok(Command::Place(piece, position)) if piece == Piece::new(Circle, Red) && position == Position::new(0, 1),
        ));
        assert!(matches!(
            Command::parse("place:star8&blue@0,2"),
            Ok(Command::Place(piece, position)) if piece == Piece::new(Star8, Blue) && position == Position::new(0, 2),
        ));
        assert!(matches!(
            Command::parse("place:star4blue0,2"),
            Err(Err::Error(err)) if err.code == ErrorKind::Tag,
        ));
        assert!(matches!(
            Command::parse("place:star4:blue&0@2"),
            Err(Err::Error(err)) if err.code == ErrorKind::Tag,
        ));
    }

    #[test]
    fn state() {
        assert!(matches!(Command::parse("state"), Ok(Command::State)));
        assert!(matches!(Command::parse("state:"), Ok(Command::State)));
    }
}
