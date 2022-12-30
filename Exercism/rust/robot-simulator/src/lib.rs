// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Rotation {
    RotateRight,
    RotateLeft,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, rotation: Rotation) -> Direction {
        match &self {
            Direction::North => if rotation == Rotation::RotateRight {Direction::East} else {Direction::West}
            Direction::East => if rotation == Rotation::RotateRight {Direction::South} else {Direction::North}
            Direction::South => if rotation == Rotation::RotateRight {Direction::West} else {Direction::East}
            Direction::West => if rotation == Rotation::RotateRight {Direction::North} else {Direction::South}
        }
    }

    fn advance(&self) -> (i32, i32) {
        match &self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

pub struct Robot {
    current_position: (i32, i32),
    current_direction:Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            current_position: (x, y),
            current_direction: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            current_position: self.current_position,
            current_direction: self.current_direction.rotate(Rotation::RotateRight),
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            current_position: self.current_position,
            current_direction: self.current_direction.rotate(Rotation::RotateLeft),
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        Self {
            current_position: (self.current_position.0 + self.current_direction.advance().0,
                               self.current_position.1 + self.current_direction.advance().1),
            current_direction: self.current_direction,
        }
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        let instruction_set = instructions.chars();

        for instruction in instruction_set {
            self = match instruction {
                'R' => self.turn_right(),
                'L' => self.turn_left(),
                'A' => self.advance(),
                _ => Self {
                    ..self
                }
            }
        };

        Self {
            ..self
        }
    }

    pub fn position(&self) -> (i32, i32) {
        self.current_position
    }

    pub fn direction(&self) -> &Direction {
       &self.current_direction
    }
}
