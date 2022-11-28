pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![EMPTY_CELL; size as usize]; size as usize];
    let mut current_cell = Cell { row: 0, column: 0 };
    let mut move_direction = Direction::Right;

    for cell_value in 1..=size * size {
        matrix[current_cell.row][current_cell.column] = cell_value;

        let next_cell = current_cell.advance(&matrix, &move_direction).or_else(|| {
            move_direction = move_direction.next();
            current_cell.advance(&matrix, &move_direction)
        });

        current_cell = match next_cell {
            Some(next_cell) => next_cell,
            _ => break,
        }
    }

    matrix
}

const EMPTY_CELL: u32 = 0;

struct Cell {
    row: usize,
    column: usize,
}

impl Cell {
    fn advance(&self, matrix: &[Vec<u32>], move_direction: &Direction) -> Option<Self> {
        let (dx, dy) = move_direction.delta();
        let next_row = checked_add_wrapped(self.row, dy, matrix.len());
        let next_column = checked_add_wrapped(self.column, dx, matrix.len());

        next_row
            .zip(next_column)
            .filter(|&(row, column)| matrix[row][column] == EMPTY_CELL)
            .map(|(row, column)| Self { row, column })
    }
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Up => (0, -1),
        }
    }
}

fn checked_add_wrapped(value: usize, delta: isize, max: usize) -> Option<usize> {
    if delta < 0 {
        value.checked_sub(delta.unsigned_abs())
    } else {
        let next_value = value + delta as usize;
        (next_value < max).then_some(next_value)
    }
}
