use std::collections::{HashMap, HashSet};

const MAX_ROUNDS: usize = 1000000000;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q14_p1.txt");
    let mut grid = Grid::from(&lines);
    let mut total = 0;
    for _ in 0..10 {
        grid.step();
        total += grid.points.len();
    }
    println!("part 1 = {total}");

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q14_p2.txt");
    let mut grid = Grid::from(&lines);
    let mut total = 0;
    for _ in 0..2025 {
        grid.step();
        total += grid.points.len();
    }
    println!("part 2 = {total}");

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q14_p3.txt");
    let center = Grid::from(&lines);
    let mut grid = Grid {
        width: 34,
        height: 34,
        ..Grid::default()
    };

    let mut repeats: HashMap<Vec<(i64, i64)>, usize> = HashMap::new();
    let mut count_per_match: Vec<(usize, usize)> = Vec::new();
    repeats.insert(grid.get_hash_vec(), 0);
    let mut prev_match = 0;
    let mut round_size = 0;
    let mut round_tiles = 0;
    for round in 1.. {
        grid.step();
        if grid.matches(&center) {
            count_per_match.push((round, grid.points.len()));
            round_tiles += grid.points.len();
        }
        let key = grid.get_hash_vec();
        if repeats.contains_key(&key) {
            prev_match = repeats[&key];
            round_size = round - 1;
            break;
        }
        repeats.insert(key, round);
    }
    println!("> repeat {prev_match} -> {round_size} rounds with {round_tiles} tiles");
    println!("> counts per match: {count_per_match:?}");
    let full_rounds = MAX_ROUNDS / round_size;
    let partial_rounds = MAX_ROUNDS % round_size;
    println!("> full = {full_rounds}, partial = {partial_rounds}");
    let mut total_tiles = full_rounds * round_tiles;
    println!("> total full = {total_tiles}");
    for (round, tiles) in count_per_match {
        if partial_rounds > round {
            total_tiles += tiles;
        } else {
            break;
        }
    }
    println!("part 3 = {total_tiles}");
    // println!("off by: {}", total_tiles.abs_diff(278388552));
}

#[derive(Debug, Default)]
struct Grid {
    points: HashSet<(i64, i64)>,
    width: i64,
    height: i64,
}

impl Grid {
    fn get_hash_vec(&self) -> Vec<(i64, i64)> {
        let mut vec = self.points.iter().copied().collect::<Vec<_>>();
        vec.sort();
        vec
    }

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

    fn _print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.points.contains(&(row, col)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
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

    fn matches(&self, other: &Grid) -> bool {
        let start_row = (self.height - other.height) / 2;
        let start_col = (self.width - other.width) / 2;
        for row in start_row..start_row + other.height {
            for col in start_col..start_col + other.width {
                if self.points.contains(&(row, col))
                    != other.points.contains(&(row - start_row, col - start_col))
                {
                    return false;
                }
            }
        }
        true
    }
}
