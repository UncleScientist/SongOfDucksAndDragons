use std::{
    collections::{BTreeMap, HashMap, HashSet},
    convert::Infallible,
    hash::Hash,
    ops::{Add, AddAssign},
    str::FromStr,
};

use macroquad::prelude::*;

#[macroquad::main("Definitely Not a Maze")]
async fn main() {
    let part1 = aoclib::read_lines("input/everybody_codes_e2025_q15_p1.txt");
    let maze = part1[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 1 = {}", maze.shortest_path());

    if std::env::var("GUI1").is_ok() {
        draw_gui(&maze, 1, 10.0).await;
    }

    let part2 = aoclib::read_lines("input/everybody_codes_e2025_q15_p2.txt");
    let maze = part2[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 2 = {}", maze.shortest_path());

    if std::env::var("GUI2").is_ok() {
        draw_gui(&maze, 100, 2.0).await;
    }
}

async fn draw_gui(maze: &Maze, steps: usize, scale: f32) {
    let neighbors = |point: &Point| {
        DIRS.iter()
            .map(|dir| *point + *dir)
            .filter(|point| !maze.walls.contains(point))
            .filter(|point| {
                point.0 >= maze.upper_left.0
                    && point.0 <= maze.lower_right.0
                    && point.1 >= maze.upper_left.1
                    && point.1 <= maze.lower_right.1
            })
            .map(|point| (point, 1))
            .collect()
    };
    let heuristic = |point: &Point| point.dist_to(&maze.end);
    let is_end = |point: &Point| maze.end == *point;

    let mut visible_astar = Astar::new(&Point(0, 0), neighbors, heuristic, is_end);

    let result = 'out: loop {
        for _ in 0..steps {
            match visible_astar.step() {
                StepResult::Answer(ans) => break 'out Some(ans),
                StepResult::Ongoing => {}
                StepResult::SearchFailure => break 'out None,
            }
        }
        maze.draw(&visible_astar.visited, None, scale).await;
    };

    let result = if let Some((_, result)) = result {
        Some(result)
    } else {
        None
    };

    while !is_mouse_button_pressed(MouseButton::Left) {
        maze.draw(&visible_astar.visited, result, scale).await;
    }

    println!("result = {result:?}");
}

#[derive(Debug)]
struct Maze {
    end: Point,
    upper_left: Point,
    lower_right: Point,
    walls: HashSet<Point>,
}

impl Maze {
    async fn draw(&self, visited: &HashSet<Point>, dist: Option<usize>, scale: f32) {
        let trans_x = self.upper_left.1.abs() as f32 * scale;
        let trans_y = self.upper_left.0.abs() as f32 * scale + 55.0;
        clear_background(BLACK);
        draw_line(0.0, 50.0, screen_width(), 50.0, 1.0, WHITE);
        for wall in &self.walls {
            draw_rectangle(
                trans_x + wall.1 as f32 * scale,
                trans_y + wall.0 as f32 * scale,
                scale - 1.0,
                scale - 1.0,
                BLUE,
            );
        }
        draw_rectangle(trans_x, trans_y, scale - 1.0, scale - 1.0, RED);
        draw_rectangle(
            trans_x + scale * self.end.1 as f32,
            trans_y + scale * self.end.0 as f32,
            scale - 1.0,
            scale - 1.0,
            RED,
        );

        for v in visited {
            draw_rectangle(
                trans_x + scale * v.1 as f32,
                trans_y + scale * v.0 as f32,
                scale - 1.0,
                scale - 1.0,
                GREEN,
            );
        }

        draw_text(
            format!("Points visited: {}", visited.len()),
            5.0,
            20.0,
            24.0,
            WHITE,
        );

        if let Some(dist) = dist {
            draw_text(format!("Shortest distance: {dist}"), 5.0, 40.0, 24.0, WHITE);
        }

        next_frame().await
    }

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

struct Astar<NODE, FN, FH, FEND> {
    queue: BTreeMap<(usize, usize), HashSet<NODE>>,
    visited: HashSet<NODE>,
    dist: HashMap<NODE, Option<usize>>,
    neighbors: FN,
    heuristic: FH,
    is_end: FEND,
}

impl<NODE, FN, FH, FEND> Astar<NODE, FN, FH, FEND>
where
    NODE: Hash + PartialEq + Eq + Copy,
    FN: Fn(&NODE) -> Vec<(NODE, usize)>,
    FH: Fn(&NODE) -> usize,
    FEND: Fn(&NODE) -> bool,
{
    fn new(start: &NODE, neighbors: FN, heuristic: FH, is_end: FEND) -> Self {
        Self {
            queue: BTreeMap::from([((0, 0), HashSet::from([*start]))]),
            visited: HashSet::new(),
            dist: HashMap::new(),
            neighbors,
            heuristic,
            is_end,
        }
    }

    fn step(&mut self) -> StepResult<NODE> {
        let Some(((h, time), pos_list)) = self.queue.pop_first() else {
            return StepResult::SearchFailure;
        };

        for pos in pos_list.iter() {
            if (self.is_end)(pos) {
                return StepResult::Answer((*pos, time));
            }
            if self.visited.insert(*pos) {
                for (new_pos, cost) in (self.neighbors)(pos) {
                    let new_time = time + cost;
                    let new_h = new_time + (self.heuristic)(&new_pos);

                    let dist_entry = self.dist.entry(new_pos).or_insert(None);
                    if let Some(dist_time) = dist_entry {
                        if new_time >= *dist_time {
                            continue;
                        }

                        if let Some(qentry) = self.queue.get_mut(&(h, *dist_time)) {
                            qentry.remove(pos);
                        }
                    }
                    *dist_entry = Some(new_time);
                    self.queue
                        .entry((new_h, new_time))
                        .or_default()
                        .insert(new_pos);
                }
            }
        }

        StepResult::Ongoing
    }
}

enum StepResult<NODE> {
    Answer((NODE, usize)),
    Ongoing,
    SearchFailure,
}
