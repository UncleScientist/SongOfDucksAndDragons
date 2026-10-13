use std::{collections::HashSet, convert::Into};

fn main() {
    let data = aoclib::read_lines("input/everybody_codes_e2025_q20_p1.txt");
    let game = Game::new(data);
    println!("Quest 20, part 1 = {}", game.count_jumps());
}

#[derive(Debug, PartialEq)]
enum Triangle {
    Trampoline,
    Empty,
}

#[derive(Debug)]
struct Game {
    grid: Vec<Vec<Triangle>>,
}

impl Game {
    fn new(data: Vec<String>) -> Self {
        Self {
            grid: data
                .iter()
                .map(|line| line.chars().map(Into::<Triangle>::into).collect())
                .collect(),
        }
    }

    fn count_jumps(&self) -> usize {
        let mut hs = HashSet::new();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == Triangle::Trampoline {
                    let from = Point::new(row, col);
                    hs.extend(self.jumps(from).iter().map(|to| PointPair::new(from, *to)));
                }
            }
        }

        hs.len()
    }

    fn jumps(&self, pos: Point) -> Vec<Point> {
        let mut result = Vec::new();

        let is_even_row = pos.row.is_multiple_of(2);
        let is_even_col = pos.col.is_multiple_of(2);

        let can_go_down = is_even_row != is_even_col;
        let can_go_up = is_even_row == is_even_col;

        if pos.row > 0 && can_go_up && self.grid[pos.row - 1][pos.col] == Triangle::Trampoline {
            result.push(pos.up());
        }

        if pos.row < self.grid.len() - 1
            && can_go_down
            && self.grid[pos.row + 1][pos.col] == Triangle::Trampoline
        {
            result.push(pos.down());
        }

        if pos.col > 0 && self.grid[pos.row][pos.col - 1] == Triangle::Trampoline {
            result.push(pos.left());
        }

        if pos.col < self.grid[pos.row].len() - 1
            && self.grid[pos.row][pos.col + 1] == Triangle::Trampoline
        {
            result.push(pos.right());
        }

        result
    }
}

impl From<char> for Triangle {
    fn from(value: char) -> Self {
        match value {
            '#' | '.' => Self::Empty,
            'T' => Self::Trampoline,
            _ => panic!("invalid char '{value}'"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PointPair {
    from: Point,
    to: Point,
}

impl PointPair {
    fn new(from: Point, to: Point) -> Self {
        if from.row < to.row || (from.row == to.row && from.col < to.col) {
            Self { from, to }
        } else {
            Self { from: to, to: from }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    fn up(&self) -> Point {
        Point {
            row: self.row - 1,
            col: self.col,
        }
    }
    fn down(&self) -> Point {
        Point {
            row: self.row + 1,
            col: self.col,
        }
    }
    fn left(&self) -> Point {
        Point {
            row: self.row,
            col: self.col - 1,
        }
    }
    fn right(&self) -> Point {
        Point {
            row: self.row,
            col: self.col + 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_input() {
        let data = aoclib::read_lines("test-input/part1");
        let game = Game::new(data);
        assert_eq!(6, game.grid.len());
    }

    #[test]
    fn test_point_pair() {
        let from = Point { row: 0, col: 5 };
        let to = Point { row: 4, col: 2 };
        let pp1 = PointPair::new(from, to);
        assert_eq!(pp1.from.row, 0);
        assert_eq!(pp1.to.col, 2);

        let pp1 = PointPair::new(to, from);
        assert_eq!(pp1.from.row, 0);
        assert_eq!(pp1.to.col, 2);
    }

    #[test]
    fn test_jumps() {
        let data = aoclib::read_lines("test-input/part1");
        let game = Game::new(data);
        assert_eq!(0, game.jumps(Point::new(1, 1)).len());

        let jumps = game.jumps(Point::new(0, 3));
        assert_eq!(3, jumps.len());

        let jumps = game.jumps(Point::new(1, 4));
        assert_eq!(1, jumps.len());
    }

    #[test]
    fn test_part_1() {
        let data = aoclib::read_lines("test-input/part1");
        let game = Game::new(data);
        assert_eq!(7, game.count_jumps());
    }
}
