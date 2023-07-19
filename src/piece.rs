#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    shape: Shape,
    color: Color,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    Circle,
    Star4,
    Diamond,
    Square,
    Star8,
    Clover,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
}

impl Piece {
    #[must_use]
    pub const fn new(shape: Shape, color: Color) -> Self {
        Self { shape, color }
    }

    #[must_use]
    pub const fn shape(&self) -> Shape {
        self.shape
    }

    #[must_use]
    pub const fn color(&self) -> Color {
        self.color
    }

    #[must_use]
    pub fn fits(
        &self,
        up: Option<&Self>,
        down: Option<&Self>,
        left: Option<&Self>,
        right: Option<&Self>,
    ) -> bool {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum MatchKind {
            Shape,
            Color,
        }

        macro_rules! match_kind_dir {
            ($dir:expr) => {{
                match $dir {
                    Some(p) => match (p.shape == self.shape, p.color == self.color) {
                        (true, false) => Some(MatchKind::Shape),
                        (false, true) => Some(MatchKind::Color),
                        _ => return false,
                    },
                    None => None,
                }
            }};
        }

        macro_rules! all_nones {
            ($($op:expr $(,)?)+) => {
                true $(&& $op.is_none())+
            }
        }

        let up_kind = match_kind_dir!(up);
        let down_kind = match_kind_dir!(down);
        let left_kind = match_kind_dir!(left);
        let right_kind = match_kind_dir!(right);

        if all_nones!(up_kind, down_kind, left_kind, right_kind) {
            return false;
        }

        if up_kind
            .zip(down_kind)
            .is_some_and(|(up_kind, down_kind)| up_kind != down_kind)
            || left_kind
                .zip(right_kind)
                .is_some_and(|(left_kind, right_kind)| left_kind != right_kind)
        {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::{Color::*, Piece, Shape::*};

    #[test]
    fn fits() {
        let center = Piece::new(Circle, Red);

        assert!(!center.fits(None, None, None, None));
        assert!(center.fits(
            Some(&Piece::new(Circle, Blue)),
            Some(&Piece::new(Circle, Yellow)),
            Some(&Piece::new(Star4, Red)),
            Some(&Piece::new(Star8, Red))
        ));
        assert!(!center.fits(
            Some(&Piece::new(Circle, Orange)),
            None,
            Some(&Piece::new(Circle, Red)),
            None,
        ));
    }
}
