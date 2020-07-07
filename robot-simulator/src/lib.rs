// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { x, y, direction }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            direction: match self.direction {
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
                Direction::North => Direction::East,
            },
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            direction: match self.direction {
                Direction::East => Direction::North,
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
            },
            ..self
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::East => Robot {
                x: self.x + 1,
                ..self
            },
            Direction::West => Robot {
                x: self.x - 1,
                ..self
            },
            Direction::North => Robot {
                y: self.y + 1,
                ..self
            },
            Direction::South => Robot {
                y: self.y - 1,
                ..self
            },
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
