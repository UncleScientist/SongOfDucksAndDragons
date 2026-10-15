use std::{collections::HashSet, convert::Into, ops::Index};

fn main() {
    let data = aoclib::read_lines("input/everybody_codes_e2025_q20_p1.txt");
    let game = Game::new(data);
    println!("Quest 20, part 1 = {}", game.count_jumps());

    let data = aoclib::read_lines("input/everybody_codes_e2025_q20_p2.txt");
    let game = Game::new(data);
    println!("Quest 20, part 2 = {}", game.count_path());

    let data = aoclib::read_lines("input/everybody_codes_e2025_q20_p3.txt");
    let game = Game::new(data);
    println!("Quest 20, part 3 = {}", game.count_bouncy_path());
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Triangle {
    Trampoline,
    Start,
    End,
    Empty,
}

impl Triangle {
    fn can_jump_to(&self) -> bool {
        !matches!(self, Self::Empty)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct Grid(Vec<Vec<Triangle>>);

impl Grid {
    fn len(&self) -> usize {
        self.0.len()
    }

    fn rotate(&self) -> Self {
        let mut grid = Vec::new();
        let width = self[0].len();
        let mut startcol = width / 2;
        let mut startrow = self.len() - 1;

        loop {
            let mut cur_col = startcol;
            let mut cur_row = startrow;
            let mut going_up = true;
            let mut grid_row = Vec::new();

            grid_row.extend(vec![Triangle::Empty; startcol - (width / 2)]);
            loop {
                grid_row.push(self[cur_row][cur_col]);
                if going_up {
                    if cur_row == 0 {
                        break;
                    }
                    cur_row -= 1;
                    going_up = false;
                } else {
                    cur_col -= 1;
                    going_up = true;
                }
            }
            grid_row.extend(vec![Triangle::Empty; startcol - (width / 2)]);
            grid.push(grid_row);

            if startrow == 0 {
                break;
            }
            startrow -= 1;
            startcol += 1;
        }

        Self(grid)
    }
}

impl Index<usize> for Grid {
    type Output = Vec<Triangle>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug)]
struct Game {
    grid: Grid,
    start: Point,
    end: Point,
}

impl Game {
    fn new(data: Vec<String>) -> Self {
        let mut grid = Grid::default();
        let mut start = Point::new(0, 0);
        let mut end = Point::new(0, 0);

        for (r, line) in data.iter().enumerate() {
            let mut row = Vec::new();
            for (c, ch) in line.chars().enumerate() {
                let item: Triangle = ch.into();
                row.push(item);
                match item {
                    Triangle::Start => start = Point::new(r, c),
                    Triangle::End => end = Point::new(r, c),
                    _ => {}
                }
            }
            grid.0.push(row);
        }

        Self { grid, start, end }
    }

    fn count_jumps(&self) -> usize {
        let mut hs = HashSet::new();

        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col].can_jump_to() {
                    let from = Point::new(row, col);
                    hs.extend(self.jumps(from).iter().map(|to| PointPair::new(from, *to)));
                }
            }
        }

        hs.len()
    }

    fn count_path(&self) -> usize {
        aoclib::ucs(
            &self.start,
            |point: &Point| self.jumps(*point).into_iter().map(|loc| (loc, 1)).collect(),
            |point: &Point| *point == self.end,
        )
        .unwrap()
        .1
    }

    fn count_bouncy_path(&self) -> usize {
        let r1 = self.grid.rotate();
        let r2 = r1.rotate();

        let rotating_grid = [&self.grid, &r1, &r2];

        aoclib::ucs(
            &(1, self.start),
            |(idx, point): &(usize, Point)| {
                Self::jumps_for_grid(*point, rotating_grid[*idx], true)
                    .into_iter()
                    .map(|loc| (((idx + 1) % 3, loc), 1))
                    .collect()
            },
            |point: &(usize, Point)| point.1 == self.end,
        )
        .unwrap()
        .1
    }

    fn jumps(&self, pos: Point) -> Vec<Point> {
        Self::jumps_for_grid(pos, &self.grid, false)
    }

    fn jumps_for_grid(pos: Point, grid: &Grid, jump_in_place: bool) -> Vec<Point> {
        let mut result = if jump_in_place && grid[pos.row][pos.col].can_jump_to() {
            vec![pos]
        } else {
            Vec::new()
        };

        let is_even_row = pos.row.is_multiple_of(2);
        let is_even_col = pos.col.is_multiple_of(2);

        let can_go_down = is_even_row != is_even_col;
        let can_go_up = is_even_row == is_even_col;

        if pos.row > 0 && can_go_up && grid[pos.row - 1][pos.col].can_jump_to() {
            result.push(pos.up());
        }

        if pos.row < grid.len() - 1 && can_go_down && grid[pos.row + 1][pos.col].can_jump_to() {
            result.push(pos.down());
        }

        if pos.col > 0 && grid[pos.row][pos.col - 1].can_jump_to() {
            result.push(pos.left());
        }

        if pos.col < grid[pos.row].len() - 1 && grid[pos.row][pos.col + 1].can_jump_to() {
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
            'S' => Self::Start,
            'E' => Self::End,
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

    #[test]
    fn test_part_2() {
        let data = aoclib::read_lines("test-input/part2");
        let game = Game::new(data);
        assert_eq!(32, game.count_path());
    }

    #[test]
    fn test_rotation() {
        let data = aoclib::read_lines("test-input/part3");
        let game = Game::new(data);
        let rotated_grid = game.grid.rotate();

        let rotated_data = aoclib::read_lines("test-input/part3.rotate1");
        let rotated_game = Game::new(rotated_data);

        assert_eq!(rotated_game.grid, rotated_grid);
    }

    #[test]
    fn test_part_3() {
        let data = aoclib::read_lines("test-input/part3");
        let game = Game::new(data);
        assert_eq!(23, game.count_bouncy_path());
    }
}
