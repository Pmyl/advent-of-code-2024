pub struct Horizontal<'a> {
    lines: &'a Vec<Vec<char>>,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl<'a> Horizontal<'a> {
    pub fn new(lines: &'a Vec<Vec<char>>) -> Self {
        Self {
            lines,
            x: 0,
            y: 0,
            width: lines[0].len() as isize,
            height: lines.len() as isize,
        }
    }
}

impl<'a> Iterator for Horizontal<'a> {
    type Item = WordSearchItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x == self.width {
            if self.y + 1 == self.height {
                None
            } else {
                self.y += 1;
                self.x = 0;
                Some(WordSearchItem::NewLine)
            }
        } else {
            let x = self.x as usize;
            let y = self.y as usize;
            self.x += 1;
            Some(WordSearchItem::Letter(self.lines[y][x], (x, y)))
        }
    }
}

pub struct Vertical<'a> {
    lines: &'a Vec<Vec<char>>,
    x: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl<'a> Vertical<'a> {
    pub fn new(lines: &'a Vec<Vec<char>>) -> Self {
        Self {
            lines,
            x: 0,
            y: 0,
            width: lines[0].len() as isize,
            height: lines.len() as isize,
        }
    }
}

impl<'a> Iterator for Vertical<'a> {
    type Item = WordSearchItem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y == self.height {
            if self.x + 1 == self.width {
                None
            } else {
                self.x += 1;
                self.y = 0;
                Some(WordSearchItem::NewLine)
            }
        } else {
            let x = self.x as usize;
            let y = self.y as usize;
            self.y += 1;
            Some(WordSearchItem::Letter(self.lines[y][x], (x, y)))
        }
    }
}

pub struct DiagonalBrTl<'a> {
    lines: &'a Vec<Vec<char>>,
    x: isize,
    x_start: isize,
    y: isize,
    width: isize,
    height: isize,
}

impl<'a> DiagonalBrTl<'a> {
    pub fn new(lines: &'a Vec<Vec<char>>) -> Self {
        Self {
            lines,
            x: 0,
            x_start: 0,
            y: (lines.len() - 1) as isize,
            width: lines[0].len() as isize,
            height: lines.len() as isize,
        }
    }
}

impl<'a> Iterator for DiagonalBrTl<'a> {
    type Item = WordSearchItem;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x as usize;
        let y = self.y as usize;

        if self.x >= 0 && self.y >= 0 {
            self.x -= 1;
            self.y -= 1;
        } else {
            self.x_start += 1;
            self.x = self.x_start.min(self.width - 1);
            self.y = self.height - 1 - (self.x_start - self.width + 1).max(0);

            let reached_virtual_double_x_end = self.x_start == self.width + self.height;

            if reached_virtual_double_x_end {
                return None;
            }

            return Some(WordSearchItem::NewLine);
        }

        Some(WordSearchItem::Letter(self.lines[y][x], (x, y)))
    }
}

pub struct DiagonalBlTr<'a> {
    lines: &'a Vec<Vec<char>>,
    x: isize,
    y: isize,
    y_start: isize,
    width: isize,
    height: isize,
}

impl<'a> DiagonalBlTr<'a> {
    pub fn new(lines: &'a Vec<Vec<char>>) -> Self {
        Self {
            lines,
            x: 0,
            y: 0,
            y_start: 0,
            width: lines[0].len() as isize,
            height: lines.len() as isize,
        }
    }
}

impl<'a> Iterator for DiagonalBlTr<'a> {
    type Item = WordSearchItem;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.x as usize;
        let y = self.y as usize;

        if self.y >= 0 && self.x < self.width {
            self.y -= 1;
            self.x += 1;
        } else {
            self.y_start += 1;
            self.y = self.y_start.min(self.height - 1);
            self.x = (self.y_start - self.height + 1).max(0);

            let reached_virtual_double_y_end = self.y_start == self.height + self.width;

            if reached_virtual_double_y_end {
                return None;
            }

            return Some(WordSearchItem::NewLine);
        }

        Some(WordSearchItem::Letter(self.lines[y][x], (x, y)))
    }
}

pub enum WordSearchItem {
    Letter(char, (usize, usize)),
    NewLine,
}
