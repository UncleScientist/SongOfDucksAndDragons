use std::{collections::HashSet, path::Path};

fn main() {
    let gb = GameBoard::parse_file("input/everybody_codes_e2025_q10_p1.txt");
    println!("part 1 = {}", gb.sheep_consumed(4));
}

struct GameBoard {
    dragon: Position,
    sheep: HashSet<Position>,
    width: i64,
    height: i64,
}

impl GameBoard {
    fn parse_file<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
        let width = lines[0].len() as i64;
        let height = lines.len() as i64;
        let mut dragon = Position(0, 0);
        let mut sheep = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'D' => {
                        dragon = Position(row as i64, col as i64);
                    }
                    'S' => {
                        sheep.insert(Position(row as i64, col as i64));
                    }
                    '.' => {}
                    _ => panic!("invalid char '{ch}'"),
                }
            }
        }

        Self {
            dragon,
            sheep,
            width,
            height,
        }
    }

    fn sheep_consumed(&self, levels: usize) -> usize {
        let mut consumed = HashSet::<Position>::new();
        let mut stack = Vec::from_iter([self.dragon]);
        for _ in 0..=levels {
            let mut next_stack = Vec::new();
            while let Some(dragon) = stack.pop() {
                if self.sheep.contains(&dragon) {
                    consumed.insert(dragon);
                }
                let jumps = dragon.next_jumps(self.width, self.height);
                for jump in jumps {
                    next_stack.push(jump);
                }
            }
            stack = next_stack;
        }

        consumed.len()
    }

    fn _print(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                let p = Position(row, col);
                if p == self.dragon {
                    print!("D");
                } else if self.sheep.contains(&p) {
                    print!("S");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i64, i64);

impl Position {
    fn new(pos: (i64, i64)) -> Self {
        Self(pos.0, pos.1)
    }

    fn next_jumps(&self, width: i64, height: i64) -> Vec<Self> {
        const JUMPS: [(i64, i64); 8] = [
            (-2, 1),
            (-1, 2),
            (1, 2),
            (2, 1),
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
        ];
        JUMPS
            .iter()
            .map(|jump| (self.0 + jump.0, self.1 + jump.1))
            .filter(|next| !(next.0 < 0 || next.1 < 0 || next.0 >= height || next.1 >= width))
            .map(Position::new)
            .collect()
    }
}
