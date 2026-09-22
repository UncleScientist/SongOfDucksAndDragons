use std::{
    collections::{BTreeMap, HashMap, HashSet},
    convert::Infallible,
    hash::Hash,
    ops::{Add, AddAssign, Mul},
    str::FromStr,
};

use macroquad::prelude::*;

#[macroquad::main("Definitely Not a Maze")]
async fn main() {
    let part1 = aoclib::read_lines("input/everybody_codes_e2025_q15_p1.txt");
    let maze = part1[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 1 = {}", maze.shortest_path());

    if std::env::var("GUI1").is_ok() {
        draw_line_gui(&maze, 1).await;
    }

    let part2 = aoclib::read_lines("input/everybody_codes_e2025_q15_p2.txt");
    let maze = part2[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 2 = {}", maze.shortest_path());

    if std::env::var("GUI2").is_ok() {
        draw_line_gui(&maze, 10).await;
    }

    let part3 = aoclib::read_lines("input/everybody_codes_e2025_q15_p3.txt");
    let maze = part3[0].parse::<Maze>().unwrap();
    println!("Quest 15, Part 3 = {}", maze.shortest_path());

    if std::env::var("GUI3").is_ok() {
        draw_line_gui(&maze, 20).await;
    }
}

async fn draw_line_gui(maze: &Maze, steps: usize) {
    let mut neighbor_list = HashMap::<Point, Vec<(Point, usize)>>::new();

    for path in &maze.path_lines {
        let (start, end) = match path {
            Line::Horizontal(Horizontal { start, end }) => (start, end),
            Line::Vertical(Vertical { start, end }) => (start, end),
        };
        neighbor_list
            .entry(*start)
            .or_default()
            .push((*end, start.dist_to(end)));
        neighbor_list
            .entry(*end)
            .or_default()
            .push((*start, start.dist_to(end)));
    }

    let mut visible_astar = Astar::new(
        &Point(0, 0),
        |point: &Point| {
            // println!("searching from {point:?}");
            if let Some(list) = neighbor_list.get(point) {
                // println!("  > {list:?}");
                list.clone()
            } else {
                // println!("  > none");
                Vec::new()
            }
        },
        |point: &Point| point.dist_to(&maze.end),
        |point: &Point| maze.end == *point,
    );

    let result = 'out: loop {
        for _ in 0..steps {
            match visible_astar.step() {
                StepResult::Answer(ans) => break 'out Some(ans),
                StepResult::Ongoing => {}
                StepResult::SearchFailure => break 'out None,
            }
        }
        maze.draw_lines(&visible_astar.visited, None).await;
    };

    let result = if let Some((_, result)) = result {
        Some(result)
    } else {
        None
    };

    while !is_mouse_button_pressed(MouseButton::Right) {
        maze.draw_lines(&visible_astar.visited, result).await;
    }
}

#[derive(Debug)]
struct Maze {
    end: Point,
    upper_left: Point,
    lower_right: Point,
    outer_points: HashSet<Point>,
    wall_lines: Vec<Line>,
    path_lines: Vec<Line>,
}

impl Maze {
    async fn draw_lines(&self, visited: &HashSet<Point>, dist: Option<usize>) {
        clear_background(BLACK);
        draw_line(0.0, 50.0, screen_width(), 50.0, 1.0, WHITE);

        let (screen_width, screen_height) = (screen_width(), screen_height());

        let maze_width = (self.lower_right.1 - self.upper_left.1) as f32 + 5.0;
        let maze_height = (self.lower_right.0 - self.upper_left.0) as f32 + 5.0;

        let xscale = screen_width / maze_width;
        let yscale = screen_height / maze_height;

        let tx = self.upper_left.1.abs() as f32 * xscale + 5.0;
        let ty = self.upper_left.0.abs() as f32 * yscale + 55.0;

        for line in &self.wall_lines {
            line.draw(tx, ty, xscale, yscale, BLUE);
        }

        for op in &self.outer_points {
            draw_circle(
                tx + xscale / 2.0 + op.1 as f32 * xscale,
                ty + yscale / 2.0 + op.0 as f32 * yscale,
                2.0,
                YELLOW,
            );
        }

        for pl in &self.path_lines {
            pl.draw(tx, ty, xscale, yscale, DARKBROWN);
        }

        for v in visited {
            draw_circle(
                tx + xscale / 2.0 + v.1 as f32 * xscale,
                ty + yscale / 2.0 + v.0 as f32 * yscale,
                2.0,
                GREEN,
            );
        }

        // start point
        draw_circle(tx + xscale / 2.0, ty + yscale / 2.0, 2.0, RED);

        // end point
        draw_circle(
            tx + xscale / 2.0 + self.end.1 as f32 * xscale,
            ty + yscale / 2.0 + self.end.0 as f32 * yscale,
            2.0,
            RED,
        );
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

        next_frame().await;
    }

    fn shortest_path(&self) -> usize {
        let mut neighbor_list = HashMap::<Point, Vec<(Point, usize)>>::new();

        for path in &self.path_lines {
            let (start, end) = match path {
                Line::Horizontal(Horizontal { start, end }) => (start, end),
                Line::Vertical(Vertical { start, end }) => (start, end),
            };
            neighbor_list
                .entry(*start)
                .or_default()
                .push((*end, start.dist_to(end)));
            neighbor_list
                .entry(*end)
                .or_default()
                .push((*start, start.dist_to(end)));
        }

        if let Some(result) = aoclib::astar(
            &Point(0, 0),
            |point: &Point| {
                // println!("searching from {point:?}");
                if let Some(list) = neighbor_list.get(point) {
                    //   println!("  > {list:?}");
                    list.clone()
                } else {
                    // println!("  > none");
                    Vec::new()
                }
            },
            |point: &Point| point.dist_to(&self.end),
            |point: &Point| self.end == *point,
        ) {
            result.1
        } else {
            0
        }
    }
}

impl FromStr for Maze {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut end = Point(0, 0);
        let mut dir = Direction::Up;
        let mut upper_left = Point(0, 0);
        let mut lower_right = Point(0, 0);
        let mut outer_points = HashSet::new();
        let mut wall_lines = Vec::new();
        let mut path_lines = HashSet::new();

        for instruction in line.split(',') {
            let turn = &instruction[0..1];
            let amount = instruction[1..].parse::<isize>().unwrap();
            let new_dir = match turn {
                "L" => dir.turn_left(),
                "R" => dir.turn_right(),
                _ => panic!("invalid turn '{turn}'"),
            };

            match (dir, new_dir) {
                (Direction::Right, Direction::Down) | (Direction::Up, Direction::Left) => {
                    outer_points.insert(Point(end.0 - 1, end.1 + 1))
                }
                (Direction::Left, Direction::Down) | (Direction::Up, Direction::Right) => {
                    outer_points.insert(Point(end.0 - 1, end.1 - 1))
                }
                (Direction::Right, Direction::Up) | (Direction::Down, Direction::Left) => {
                    outer_points.insert(Point(end.0 + 1, end.1 + 1))
                }
                (Direction::Left, Direction::Up) | (Direction::Down, Direction::Right) => {
                    outer_points.insert(Point(end.0 + 1, end.1 - 1))
                }
                _ => unreachable!(),
            };

            dir = new_dir;
            let start = end;
            end += dir * amount;
            wall_lines.push(Line::from_points(start, end));

            upper_left = Point(upper_left.0.min(end.0), upper_left.1.min(end.1));
            lower_right = Point(lower_right.0.max(end.0), lower_right.1.max(end.1));
        }

        outer_points.insert(Point(0, 0));
        outer_points.insert(end);

        let valid_line = |line: &Line| !wall_lines.iter().any(|wall| line.intersects_with(wall));

        for point1 in &outer_points {
            for point2 in &outer_points {
                if point1 == point2 {
                    continue;
                }
                if point1.0 == point2.0 || point1.1 == point2.1 {
                    let line = Line::from_points(*point1, *point2);
                    if !valid_line(&line) {
                        continue;
                    }
                    path_lines.insert(line);
                } else {
                    //  a---------m
                    //  |         |
                    //  |         |
                    //  n---------b
                    let a = Point(point1.0.min(point2.0), point1.1.min(point2.1));
                    let b = Point(point1.0.max(point2.0), point1.1.max(point2.1));

                    let m = Point(point1.0.min(point2.0), point1.1.max(point2.1));
                    let n = Point(point1.0.max(point2.0), point1.1.min(point2.1));

                    let am = Line::from_points(a, m);
                    let mb = Line::from_points(m, b);
                    let an = Line::from_points(a, n);
                    let nb = Line::from_points(n, b);

                    if valid_line(&am) {
                        path_lines.insert(am);
                    }
                    if valid_line(&mb) {
                        path_lines.insert(mb);
                    }
                    if valid_line(&an) {
                        path_lines.insert(an);
                    }
                    if valid_line(&nb) {
                        path_lines.insert(nb);
                    }
                }
            }
        }

        let path_lines = path_lines.into_iter().collect();
        // println!("{path_lines:?}");

        Ok(Self {
            end,
            upper_left,
            lower_right,
            outer_points,
            wall_lines,
            path_lines,
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

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

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

impl Mul<isize> for Direction {
    type Output = Point;

    fn mul(self, rhs: isize) -> Self::Output {
        match self {
            Direction::Up => Point(-rhs, 0),
            Direction::Down => Point(rhs, 0),
            Direction::Left => Point(0, -rhs),
            Direction::Right => Point(0, rhs),
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

    #[test]
    fn test_horiz_intersects_vert() {
        let h = Horizontal::new(Point(0, 0), Point(0, -6));
        let v1 = Vertical::new(Point(6, 3), Point(-6, 3)); // to the right
        let v2 = Vertical::new(Point(6, -3), Point(-6, -3)); // goes through
        let v3 = Vertical::new(Point(6, -12), Point(-6, -12)); // to the left
        assert!(!h.intersects_vert(&v1));
        assert!(h.intersects_vert(&v2));
        assert!(!h.intersects_vert(&v3));
    }

    #[test]
    fn test_horiz_intersects_horiz() {
        let h1 = Horizontal::new(Point(0, 0), Point(0, 10));
        let h2 = Horizontal::new(Point(0, -4), Point(0, 14));
        let h3 = Horizontal::new(Point(0, -4), Point(0, 5));
        let h4 = Horizontal::new(Point(0, 7), Point(0, 14));
        let h5 = Horizontal::new(Point(0, -10), Point(0, -5));
        let h6 = Horizontal::new(Point(0, 20), Point(0, 30));
        assert!(h1.intersects_horiz(&h2));
        assert!(h1.intersects_horiz(&h3));
        assert!(h1.intersects_horiz(&h4));
        assert!(!h1.intersects_horiz(&h5));
        assert!(!h1.intersects_horiz(&h6));
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

#[derive(Debug, Hash, PartialEq, Eq)]
enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

impl Line {
    fn from_points(start: Point, end: Point) -> Self {
        if start.0 == end.0 {
            Self::Horizontal(Horizontal::new(start, end))
        } else {
            Self::Vertical(Vertical::new(start, end))
        }
    }

    fn draw(&self, tx: f32, ty: f32, xscale: f32, yscale: f32, color: Color) {
        let (x1, y1, x2, y2) = match self {
            Line::Horizontal(Horizontal { start, end }) => (start.1, start.0, end.1, end.0),
            Line::Vertical(Vertical { start, end }) => (start.1, start.0, end.1, end.0),
        };
        draw_line(
            tx + xscale / 2.0 + x1 as f32 * xscale,
            ty + yscale / 2.0 + y1 as f32 * yscale,
            tx + xscale / 2.0 + x2 as f32 * xscale,
            ty + yscale / 2.0 + y2 as f32 * yscale,
            1.0,
            color,
        );
    }

    fn intersects_with(&self, wall: &Line) -> bool {
        match (self, wall) {
            (Line::Horizontal(h1), Line::Horizontal(h2)) => h1.intersects_horiz(h2),
            (Line::Horizontal(h), Line::Vertical(v)) => h.intersects_vert(v),
            (Line::Vertical(v), Line::Horizontal(h)) => v.intersects_horiz(h),
            (Line::Vertical(v1), Line::Vertical(v2)) => v1.intersects_vert(v2),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Horizontal {
    start: Point,
    end: Point,
}

impl Horizontal {
    fn new(p1: Point, p2: Point) -> Self {
        assert_eq!(p1.0, p2.0);
        Self {
            start: Point(p1.0, p1.1.min(p2.1)),
            end: Point(p1.0, p1.1.max(p2.1)),
        }
    }

    fn intersects_vert(&self, vert: &Vertical) -> bool {
        self.start.1 < vert.start.1
            && self.end.1 > vert.end.1
            && self.start.0 > vert.start.0
            && self.end.0 < vert.end.0
    }

    fn intersects_horiz(&self, other: &Horizontal) -> bool {
        self.start.0 == other.start.0
            && ((self.start.1 < other.start.1 && self.end.1 > other.end.1)
                || (self.start.1 < other.start.1 && self.end.1 > other.start.1)
                || (self.start.1 < other.end.1 && self.end.1 > other.end.1)
                || (self.start.1 > other.start.1 && self.end.1 < other.end.1))
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct Vertical {
    start: Point,
    end: Point,
}

impl Vertical {
    fn new(p1: Point, p2: Point) -> Self {
        assert_eq!(p1.1, p2.1);
        Self {
            start: Point(p1.0.min(p2.0), p1.1),
            end: Point(p1.0.max(p2.0), p1.1),
        }
    }

    fn intersects_horiz(&self, horiz: &Horizontal) -> bool {
        horiz.intersects_vert(self)
    }

    fn intersects_vert(&self, other: &Vertical) -> bool {
        self.start.1 == other.start.1
            && ((self.start.0 < other.start.0 && self.end.0 > other.end.0)
                || (self.start.0 < other.start.0 && self.end.0 > other.start.0)
                || (self.start.0 < other.end.0 && self.end.0 > other.end.0))
    }
}
