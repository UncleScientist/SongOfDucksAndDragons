use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q14_p1.txt");
    let mut grid = Grid::from(&lines);
    let mut total = 0;
    for _ in 0..10 {
        grid.step();
        total += grid.points.len();
    }
    println!("part 1 = {total}");
}

#[derive(Debug)]
struct Grid {
    points: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
}

impl Grid {
    fn from<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut points = HashSet::new();
        let width = lines.len() as i64;
        let height = lines[0].as_ref().len() as i64;
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.as_ref().chars().enumerate() {
                if ch == '#' {
                    points.insert((row as i64, col as i64));
                }
            }
        }
        Self {
            points,
            width,
            height,
        }
    }

    fn step(&mut self) {
        let mut next_set = HashSet::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let count = [(-1, -1), (-1, 1), (1, 1), (1, -1)]
                    .iter()
                    .map(|delta| (delta.0 + row, delta.1 + col))
                    .filter(|point| self.points.contains(point))
                    .count();
                if (self.points.contains(&(row, col)) && !count.is_multiple_of(2))
                    || (!self.points.contains(&(row, col)) && count.is_multiple_of(2))
                {
                    next_set.insert((row, col));
                }
            }
        }
        self.points = next_set;
    }
}
