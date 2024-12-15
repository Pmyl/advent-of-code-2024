pub mod days;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(usize, usize);

impl Position {
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

    fn move_by(&self, distance: &Distance, width: usize, height: usize) -> Option<Position> {
        let target_x = self.0 as isize + distance.0;
        let target_y = self.1 as isize + distance.1;

        if target_x >= 0 && target_x < width as isize && target_y >= 0 && target_y < height as isize
        {
            Some(Position(target_x as usize, target_y as usize))
        } else {
            None
        }
    }

    fn move_by_wrapping(&self, distance: &Distance, width: usize, height: usize) -> Position {
        let target_x = (self.0 as isize + distance.0).rem_euclid(width as isize) as usize;
        let target_y = (self.1 as isize + distance.1).rem_euclid(height as isize) as usize;

        Position(target_x, target_y)
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
