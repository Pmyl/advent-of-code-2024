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
