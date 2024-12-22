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

    fn right_bounded(&self, width: usize) -> Option<Position> {
        if self.0 < width - 1 {
            Some(Position(self.0 + 1, self.1))
        } else {
            None
        }
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

    fn down_bounded(&self, height: usize) -> Option<Position> {
        if self.1 < height - 1 {
            Some(Position(self.0, self.1 + 1))
        } else {
            None
        }
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

    pub fn from_direction(direction: &Direction) -> Self {
        match direction {
            Direction::Up => Distance(0, -1),
            Direction::Down => Distance(0, 1),
            Direction::Left => Distance(-1, 0),
            Direction::Right => Distance(1, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub fn debug_pause() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
}
