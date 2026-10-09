// Idea 1:
//  - For any given distance (x-coord), if the distance is even, then
//    we can only pass through even-numbered heights. Odd distance = odd height.
// Idea 2:
//  - For any given distance, there's a min and max height that can be reached

use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = aoclib::read_lines("input/everybody_codes_e2025_q19_p1.txt");
    let walls = data
        .iter()
        .map(|line| line.parse::<Wall>().unwrap())
        .collect::<Vec<_>>();
    let game = Game::new(walls);
    println!("Quest 19, part 1 = {}", game.find_path());
}

struct Game {
    walls: Vec<Wall>,
}

impl Game {
    fn new(walls: Vec<Wall>) -> Self {
        Self { walls }
    }

    fn find_path(&self) -> usize {
        let bird = Bird::new(0, 0);
        aoclib::astar(
            &(0, bird),
            |pos: &(usize, Bird)| {
                let dist = self.walls[pos.0].dist - pos.1.x;
                if let Some(range) = pos.1.y_range(&self.walls[pos.0]) {
                    (range.0..=range.1)
                        .step_by(2)
                        .map(|height| {
                            let cost = pos.1.flaps_to(height, dist);
                            let new_x = pos.1.x + dist;
                            let new_y = height;
                            ((pos.0 + 1, Bird::new(new_x, new_y)), cost)
                        })
                        .collect::<Vec<_>>()
                } else {
                    Vec::new()
                }
            },
            |_| 1,
            |pos: &(usize, Bird)| pos.0 >= self.walls.len(),
        )
        .unwrap()
        .1
    }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
struct Bird {
    x: usize,
    y: usize,
}

impl Bird {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn y_range(&self, wall: &Wall) -> Option<(usize, usize)> {
        let delta = wall.dist - self.x;
        let min_y = self.y.saturating_sub(delta);
        let max_y = self.y + delta;
        if max_y < wall.bottom || min_y > wall.bottom + wall.opening {
            None
        } else {
            let lower = if wall.dist.is_multiple_of(2) == wall.bottom.is_multiple_of(2) {
                min_y.max(wall.bottom)
            } else {
                min_y.max(wall.bottom + 1)
            };

            let wall_top = wall.bottom + wall.opening - 1;
            let upper = if wall.dist.is_multiple_of(2) == wall_top.is_multiple_of(2) {
                max_y.min(wall_top)
            } else {
                max_y.min(wall_top - 1)
            };

            // assert!(lower <= upper);
            Some((lower, upper))
        }
    }

    fn flaps_to(&self, height: usize, dist: usize) -> usize {
        dist - ((self.y + dist) - height) / 2
    }
}

#[derive(Debug, Default)]
struct Wall {
    dist: usize,
    bottom: usize,
    opening: usize,
}

impl Wall {
    #[cfg(test)]
    fn make(dist: usize, bottom: usize, opening: usize) -> Self {
        Self {
            dist,
            bottom,
            opening,
        }
    }
}

impl FromStr for Wall {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut nums = line.split(',');
        let dist = nums.next().unwrap().parse::<usize>().unwrap();
        let bottom = nums.next().unwrap().parse::<usize>().unwrap();
        let opening = nums.next().unwrap().parse::<usize>().unwrap();
        Ok(Wall {
            dist,
            bottom,
            opening,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_odd_distance() {
        let start = Bird::default();
        let end = Wall::make(7, 7, 2);

        assert_eq!(Some((7, 7)), start.y_range(&end));
    }

    #[test]
    fn test_wall_ranges() {
        let bird = Bird::new(7, 7);
        let wall = Wall::make(12, 0, 4);
        assert_eq!(Some((2, 2)), bird.y_range(&wall));
    }

    #[test]
    fn test_wall_range_2() {
        let bird = Bird::new(15, 5);
        let wall = Wall::make(24, 1, 6);
        assert_eq!(Some((2, 6)), bird.y_range(&wall));
    }

    #[test]
    fn test_part_1() {
        let data = aoclib::read_lines("test-input/part1");
        let walls = data
            .iter()
            .map(|line| line.parse::<Wall>().unwrap())
            .collect::<Vec<_>>();
        let game = Game::new(walls);
        assert_eq!(24, game.find_path());
    }
}
