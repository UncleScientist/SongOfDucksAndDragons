// Idea 1:
//  - For any given distance (x-coord), if the distance is even, then
//    we can only pass through even-numbered heights. Odd distance = odd height.
// Idea 2:
//  - For any given distance, there's a min and max height that can be reached

use std::{collections::HashMap, convert::Infallible, str::FromStr};

fn main() {
    let data = aoclib::read_lines("input/everybody_codes_e2025_q19_p1.txt");
    let game = Game::new(data);
    println!("Quest 19, part 1 = {}", game.find_path());
}

struct Game {
    walls: Vec<GapRange>,
}

impl Game {
    fn new(wall_list: Vec<String>) -> Self {
        let mut hm = HashMap::<usize, GapRange>::new();

        for wall in wall_list {
            let w = wall.parse::<Wall>().unwrap();
            let entry = hm.entry(w.dist).or_default();
            entry.dist = w.dist;
            entry.gaps.push(Gap::new(w.bottom, w.opening));
        }

        let mut walls = hm.into_values().collect::<Vec<_>>();
        walls.sort_by_key(|wall| wall.dist);

        Self { walls }
    }

    fn find_path(&self) -> usize {
        let bird = Bird::new(0, 0);
        aoclib::astar(
            &(0, bird),
            |pos: &(usize, Bird)| {
                let dist = self.walls[pos.0].dist - pos.1.x;
                let highest = pos.1.y + dist;
                let lowest = pos.1.y.saturating_sub(dist).parity(highest);
                self.walls[pos.0]
                    .iter(lowest, highest)
                    .map(|height| {
                        let cost = pos.1.flaps_to(height, dist);
                        let new_x = pos.1.x + dist;
                        let new_y = height;
                        ((pos.0 + 1, Bird::new(new_x, new_y)), cost)
                    })
                    .collect::<Vec<_>>()
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

    fn flaps_to(&self, height: usize, dist: usize) -> usize {
        dist - ((self.y + dist) - height) / 2
    }
}

#[derive(Debug, Default)]
struct GapRange {
    dist: usize,
    gaps: Vec<Gap>,
}

impl GapRange {
    fn iter(&self, lowest: usize, highest: usize) -> GapRangeIter<'_> {
        let lowest = if lowest < self.gaps[0].bottom {
            self.gaps[0].bottom
                + (lowest.is_multiple_of(2) != self.gaps[0].bottom.is_multiple_of(2)) as usize
        } else {
            lowest
        };
        GapRangeIter {
            gap_range: self,
            cur_index: 0,
            next_val: lowest,
            highest,
        }
    }
}

impl Iterator for GapRangeIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next_val > self.highest || self.cur_index >= self.gap_range.gaps.len() {
            None
        } else {
            let val = self.next_val;
            self.next_val += 2;
            while self.cur_index < self.gap_range.gaps.len()
                && self.next_val
                    >= self.gap_range.gaps[self.cur_index].bottom
                        + self.gap_range.gaps[self.cur_index].opening
            {
                self.cur_index += 1;
                if self.cur_index < self.gap_range.gaps.len() {
                    let next_val = self.gap_range.gaps[self.cur_index].bottom;
                    self.next_val = if next_val.is_multiple_of(2) != val.is_multiple_of(2) {
                        next_val + 1
                    } else {
                        next_val
                    }
                }
            }

            Some(val)
        }
    }
}

#[derive(Debug)]
struct GapRangeIter<'a> {
    gap_range: &'a GapRange,
    cur_index: usize,
    next_val: usize,
    highest: usize,
}

#[derive(Debug, Default)]
struct Gap {
    bottom: usize,
    opening: usize,
}

impl Gap {
    fn new(bottom: usize, opening: usize) -> Self {
        Self { bottom, opening }
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

trait AdjustParity {
    fn parity(&self, alt: Self) -> Self;
}

impl AdjustParity for usize {
    fn parity(&self, alt: Self) -> Self {
        if self.is_multiple_of(2) != alt.is_multiple_of(2) {
            self + 1
        } else {
            *self
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = aoclib::read_lines("test-input/part1");
        let game = Game::new(data);
        assert_eq!(24, game.find_path());
    }

    #[test]
    fn test_gap_iter_even() {
        let range = GapRange {
            dist: 1,
            gaps: vec![Gap::new(4, 8), Gap::new(15, 4), Gap::new(28, 12)],
        };
        let mut iter = range.iter(4, 32);
        assert_eq!(Some(4), iter.next());
        assert_eq!(Some(6), iter.next());
        assert_eq!(Some(8), iter.next());
        assert_eq!(Some(10), iter.next());
        assert_eq!(Some(16), iter.next());
        assert_eq!(Some(18), iter.next());
        assert_eq!(Some(28), iter.next());
        assert_eq!(Some(30), iter.next());
        assert_eq!(Some(32), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_gap_iter_odd() {
        let range = GapRange {
            dist: 1,
            gaps: vec![Gap::new(4, 8), Gap::new(15, 4), Gap::new(28, 12)],
        };
        let mut iter = range.iter(5, 33);
        assert_eq!(Some(5), iter.next());
        assert_eq!(Some(7), iter.next());
        assert_eq!(Some(9), iter.next());
        assert_eq!(Some(11), iter.next());
        assert_eq!(Some(15), iter.next());
        assert_eq!(Some(17), iter.next());
        assert_eq!(Some(29), iter.next());
        assert_eq!(Some(31), iter.next());
        assert_eq!(Some(33), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_parity() {
        assert_eq!(4, 3.parity(6));
        assert_eq!(17, 17.parity(15));
        assert_eq!(17, 16.parity(95));
    }
}
