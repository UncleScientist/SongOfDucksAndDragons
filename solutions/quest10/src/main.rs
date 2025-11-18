use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() {
    let gb = GameBoard::parse_file("input/everybody_codes_e2025_q10_p1.txt");
    println!("part 1 = {}", gb.sheep_consumed(4));

    let gb = GameBoard::parse_file("input/everybody_codes_e2025_q10_p2.txt");
    println!("part 2 = {}", gb.moving_sheep_consumed(20));

    let gb = GameBoard::parse_file("input/everybody_codes_e2025_q10_p3.txt");
    let mut cache = Cache::default();
    let initial_state = gb.get_initial_state();
    println!("part 3 = {}", cache.find_solutions(&gb, &initial_state));
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Turn {
    Dragon,
    Sheep,
}

#[derive(Eq, Clone, Debug)]
struct GameState {
    dragon: Position,
    sheep: Vec<Position>,
    debug: String,
    turn: Turn,
}

impl std::hash::Hash for GameState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.dragon.hash(state);
        self.sheep.hash(state);
        // self.debug.hash(state);
        self.turn.hash(state);
    }
}

impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.dragon == other.dragon && self.sheep == other.sheep && self.turn == other.turn
    }
}

struct GameBoard {
    dragon: Position,
    sheep: HashSet<Position>,
    hideout: HashSet<Position>,
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
        let mut hideout = HashSet::new();
        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    'D' => {
                        dragon = Position(row as i64, col as i64);
                    }
                    'S' => {
                        sheep.insert(Position(row as i64, col as i64));
                    }
                    '#' => {
                        hideout.insert(Position(row as i64, col as i64));
                    }
                    '.' => {}
                    _ => panic!("invalid char '{ch}'"),
                }
            }
        }

        Self {
            dragon,
            sheep,
            hideout,
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

    fn moving_sheep_consumed(&self, levels: usize) -> usize {
        let mut total_eaten = 0;
        let mut sheep = self.sheep.clone();
        let mut dragons = HashSet::from_iter(self.dragon.next_jumps(self.width, self.height));

        for _level in 0..levels {
            let mut next_stack = HashSet::new();

            // dragon's move
            for dragon in &dragons {
                if sheep.contains(dragon) && !self.hideout.contains(dragon) {
                    total_eaten += 1;
                    sheep.remove(dragon);
                }
                let jumps = dragon.next_jumps(self.width, self.height);
                for jump in jumps {
                    next_stack.insert(jump);
                }
            }

            // sheep move
            sheep = sheep
                .into_iter()
                .map(|sheep| Position(sheep.0 + 1, sheep.1))
                .filter(|sheep| sheep.0 < self.height)
                .collect();
            for d in &dragons {
                if sheep.contains(d) && !self.hideout.contains(d) {
                    sheep.remove(d);
                    total_eaten += 1;
                }
            }

            dragons = next_stack;
        }

        total_eaten
    }

    fn next_moves(&self, state: &GameState) -> Option<Vec<GameState>> {
        match state.turn {
            Turn::Dragon => Some(
                state
                    .dragon
                    .next_jumps(self.width, self.height)
                    .into_iter()
                    .map(|dragon| GameState {
                        dragon,
                        sheep: state
                            .sheep
                            .iter()
                            .filter(|sheep| **sheep != dragon || self.hideout.contains(*sheep))
                            .copied()
                            .collect(),
                        debug: format!(
                            "{} D>{}{}",
                            state.debug,
                            (dragon.1 as u8 + b'A') as char,
                            dragon.0 + 1
                        ),
                        turn: Turn::Sheep,
                    })
                    .collect(),
            ),
            Turn::Sheep => {
                let mut result = Vec::new();
                let mut can_escape = false;
                for index in 0..state.sheep.len() {
                    let mut next_sheep = state.sheep.clone();
                    let next_pos = Position(next_sheep[index].0 + 1, next_sheep[index].1);
                    if next_pos.0 == self.height {
                        can_escape = true;
                    } else if next_pos != state.dragon || self.hideout.contains(&next_pos) {
                        let debug = format!(
                            "{} S>{}{}",
                            state.debug,
                            (next_sheep[index].1 as u8 + b'A') as char,
                            next_sheep[index].0 + 1
                        );
                        next_sheep[index] = next_pos;
                        result.push(GameState {
                            dragon: state.dragon,
                            sheep: next_sheep,
                            turn: Turn::Dragon,
                            debug,
                        });
                    }
                }
                if can_escape && result.is_empty() {
                    None
                } else {
                    Some(result)
                }
            }
        }
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

    fn get_initial_state(&self) -> GameState {
        GameState {
            dragon: self.dragon,
            sheep: self.sheep.iter().copied().collect(),
            debug: "".to_string(),
            turn: Turn::Sheep,
        }
    }
}

#[derive(Default)]
struct Cache {
    cache: HashMap<GameState, usize>,
}

impl Cache {
    fn find_solutions(&mut self, board: &GameBoard, state: &GameState) -> usize {
        if let Some(answer) = self.cache.get(state) {
            return *answer;
        }

        self.cache.insert(state.clone(), 0);

        let count = if state.sheep.is_empty() {
            1
        } else {
            match board.next_moves(state) {
                None => 0,
                Some(mut moves) => {
                    if moves.is_empty() {
                        assert_eq!(state.turn, Turn::Sheep);
                        moves = board
                            .next_moves(&GameState {
                                turn: Turn::Dragon,
                                ..state.clone()
                            })
                            .expect("dragon couldn't move");
                    }
                    moves
                        .into_iter()
                        .map(|state| self.find_solutions(board, &state))
                        .sum::<usize>()
                }
            }
        };

        self.cache.insert(state.clone(), count);
        count
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example3_1() {
        let gb = GameBoard::parse_file("input/test3_1.txt");
        let mut cache = Cache::default();
        let initial_state = gb.get_initial_state();
        assert_eq!(15, cache.find_solutions(&gb, &initial_state));
    }

    #[test]
    fn test_example3_2() {
        let gb = GameBoard::parse_file("input/test3_2.txt");
        let mut cache = Cache::default();
        let initial_state = gb.get_initial_state();
        assert_eq!(8, cache.find_solutions(&gb, &initial_state));
    }

    #[test]
    fn test_example3_3() {
        let gb = GameBoard::parse_file("input/test3_3.txt");
        let mut cache = Cache::default();
        let initial_state = gb.get_initial_state();
        assert_eq!(44, cache.find_solutions(&gb, &initial_state));
    }

    #[test]
    fn test_example3_4() {
        let gb = GameBoard::parse_file("input/test3_4.txt");
        let mut cache = Cache::default();
        let initial_state = gb.get_initial_state();
        assert_eq!(4406, cache.find_solutions(&gb, &initial_state));
    }

    #[test]
    fn test_example3_5() {
        let gb = GameBoard::parse_file("input/test3_5.txt");
        let mut cache = Cache::default();
        let initial_state = gb.get_initial_state();
        assert_eq!(13033988838, cache.find_solutions(&gb, &initial_state));
    }
}
