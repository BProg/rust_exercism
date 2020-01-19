// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_i8(&self) -> i8 {
        match self {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }

    fn from_i8(v: i8) -> Direction {
        match v {
            0 | 4 ..= std::i8::MAX => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            std::i8::MIN ..= -1 | 3 => Direction::West,
        }
    }
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: Direction::from_i8(self.direction.to_i8() + 1),
            ..self
        }

    }
    pub fn turn_left(self) -> Self {
        Self {
            direction: Direction::from_i8(self.direction.to_i8() - 1),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Self {
            x: match self.direction {
                Direction::East => self.x + 1,
                Direction::West => self.x - 1,
                _ => self.x
            },
            y: match self.direction {
                Direction::North => self.y + 1,
                Direction::South => self.y - 1,
                _ => self.y
            },
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, ch| {
            match ch {
                'R' => acc.turn_right(),
                'L' => acc.turn_left(),
                'A' => acc.advance(),
                _ => acc
            }
        })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
