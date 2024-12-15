pub mod days;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(usize, usize);

pub trait Movable {
    fn left(&self) -> Option<Position>;
    fn right(&self) -> Position;
    fn up(&self) -> Option<Position>;
    fn down(&self) -> Position;
}

impl Movable for Position {
    fn left(&self) -> Option<Position> {
        if self.0 > 0 {
            Some(Position(self.0 - 1, self.1))
        } else {
            None
        }
    }

    fn right(&self) -> Position {
        Position(self.0 + 1, self.1)
    }

    fn up(&self) -> Option<Position> {
        if self.1 > 0 {
            Some(Position(self.0, self.1 - 1))
        } else {
            None
        }
    }

    fn down(&self) -> Position {
        Position(self.0, self.1 + 1)
    }
}

#[derive(Debug)]
pub struct Distance(isize, isize);

impl Distance {
    pub fn between(position1: &Position, position2: &Position) -> Self {
        Self(
            position2.0 as isize - position1.0 as isize,
            position2.1 as isize - position1.1 as isize,
        )
    }

    pub fn multiply(&self, n: usize) -> Distance {
        Distance(self.0 * n as isize, self.1 * n as isize)
    }
}
