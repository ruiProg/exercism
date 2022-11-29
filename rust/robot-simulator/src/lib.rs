use int_enum::IntEnum;

#[repr(u32)]
#[derive(PartialEq, Eq, Debug, Clone, Copy, IntEnum)]
pub enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

pub struct Robot {
    position: Position,
    facing_direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot {
            position: (x, y),
            facing_direction: d,
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            facing_direction: Self::next_direction(self.facing_direction, 1),
            ..self
        }
    }

    pub fn turn_left(self) -> Self {
        Self {
            facing_direction: Self::next_direction(
                self.facing_direction,
                Self::DIRECTION_DATA.len() - 1,
            ),
            ..self
        }
    }

    pub fn advance(self) -> Self {
        Self {
            position: add_position(&self.position, &self.advance_delta()),
            ..self
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |next_pos, action| match action {
                'R' => next_pos.turn_right(),
                'L' => next_pos.turn_left(),
                'A' => next_pos.advance(),
                _ => next_pos,
            })
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.facing_direction
    }

    fn next_direction(direction: Direction, delta: usize) -> Direction {
        let pos = (direction.int_value() as usize + delta) % Self::DIRECTION_DATA.len();
        let (direction_data, _) = Self::DIRECTION_DATA[pos];
        direction_data
    }

    fn advance_delta(&self) -> Position {
        let (_, delta) = Self::DIRECTION_DATA[self.facing_direction.int_value() as usize];
        delta
    }

    const DIRECTION_DATA: [(Direction, Position); 4] = [
        (Direction::North, (0, 1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, -1)),
        (Direction::West, (-1, 0)),
    ];
}

type Position = (i32, i32);

fn add_position(position: &Position, delta: &Position) -> Position {
    (position.0 + delta.0, position.1 + delta.1)
}
