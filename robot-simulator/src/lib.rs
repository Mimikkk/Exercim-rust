
#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot{
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Robot { position: (x, y), direction }
    }

    pub fn turn_left(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::East => self.direction = Direction::North,
            Direction::West => self.direction = Direction::South,
        };
        self
    }

    pub fn turn_right(mut self) -> Self {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::East => self.direction = Direction::South,
            Direction::West => self.direction = Direction::North,
        };
        self
    }

    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.position.1 = self.position.1 + 1,
            Direction::South => self.position.1 = self.position.1 - 1,
            Direction::East => self.position.0 = self.position.0 + 1,
            Direction::West => self.position.0 =  self.position.0 - 1,
        };
        self
    }

    pub fn instructions(mut self, instructions: &str) -> Self {
        for op in instructions.chars() {
            match op {
                'A' => {self = self.advance(); },
                'L' => {self = self.turn_left(); },
                'R' => {self = self.turn_right(); },
                _ => { panic!("Unknown operation: '{}'", op); },
            };
        }
        self
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
