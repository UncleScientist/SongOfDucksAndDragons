use std::{
    collections::HashSet,
    convert::Infallible,
    ops::{Add, AddAssign},
    str::FromStr,
};

fn main() {
    let part1 = aoclib::read_lines("input/everybody_codes_e2025_q15_p1.txt");
    let maze = part1[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 1 = {}", maze.shortest_path());
}

#[derive(Debug)]
struct Maze {
    end: Point,
    upper_left: Point,
    lower_right: Point,
    walls: HashSet<Point>,
}

impl Maze {
    fn shortest_path(&self) -> usize {
        aoclib::astar(
            &Point(0, 0),
            |point: &Point| {
                DIRS.iter()
                    .map(|dir| *point + *dir)
                    .filter(|point| !self.walls.contains(point))
                    .filter(|point| {
                        point.0 >= self.upper_left.0
                            && point.0 <= self.lower_right.0
                            && point.1 >= self.upper_left.1
                            && point.1 <= self.lower_right.1
                    })
                    .map(|point| (point, 1))
                    .collect()
            },
            |point: &Point| point.dist_to(&self.end),
            |point: &Point| self.end == *point,
        )
        .unwrap()
        .1

        /*
        pub fn astar<T, S>(
            start: &T,
            neighbors: impl Fn(&T) -> Vec<(T, S)>,
            heuristic: impl Fn(&T) -> S,
            is_end: impl Fn(&T) -> bool,
        ) -> Option<(T, S)>
            */
    }
}

impl FromStr for Maze {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut end = Point(0, 0);
        let mut walls = HashSet::new();
        let mut dir = Direction::Up;
        let mut upper_left = Point(0, 0);
        let mut lower_right = Point(0, 0);

        for instruction in line.split(',') {
            let turn = &instruction[0..1];
            let amount = instruction[1..].parse::<isize>().unwrap();
            dir = match turn {
                "L" => dir.turn_left(),
                "R" => dir.turn_right(),
                _ => panic!("invalid turn '{turn}'"),
            };
            for _ in 0..amount {
                end += dir;
                walls.insert(end);
            }

            upper_left = Point(upper_left.0.min(end.0), upper_left.1.min(end.1));
            lower_right = Point(lower_right.0.max(end.0), lower_right.1.max(end.1));
        }

        walls.remove(&end);

        Ok(Self {
            end,
            upper_left,
            lower_right,
            walls,
        })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct Point(isize, isize); // Row, Col

impl Point {
    fn dist_to(&self, end: &Point) -> usize {
        self.0.abs_diff(end.0) + self.1.abs_diff(end.1)
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Up => Point(self.0 - 1, self.1),
            Direction::Down => Point(self.0 + 1, self.1),
            Direction::Left => Point(self.0, self.1 - 1),
            Direction::Right => Point(self.0, self.1 + 1),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Up => self.0 -= 1,
            Direction::Down => self.0 += 1,
            Direction::Left => self.1 -= 1,
            Direction::Right => self.1 += 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create_maze_from_string() {
        let maze = "R3,R4,L3,L4,R3,R6,R9".parse::<Maze>().unwrap();
        assert_eq!(Point(6, 0), maze.end);
    }

    #[test]
    fn test_find_shortest_path_1() {
        let maze = "R3,R4,L3,L4,R3,R6,R9".parse::<Maze>().unwrap();
        assert_eq!(6, maze.shortest_path());
    }

    #[test]
    fn test_find_shortest_path_2() {
        let maze = "L6,L3,L6,R3,L6,L3,L3,R6,L6,R6,L6,L6,R3,L3,L3,R3,R3,L6,L6,L3"
            .parse::<Maze>()
            .unwrap();
        assert_eq!(16, maze.shortest_path());
    }
}
