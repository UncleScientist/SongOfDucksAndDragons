use std::{collections::VecDeque, ops::Range};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p1.txt");
    let dial = Dial::build_from(&lines);
    println!("part 1 = {}", dial.turn_clockwise(2025));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p2.txt");
    let dial = RangeDial::build_from_ranges(&lines);
    println!("part 2 = {}", dial.turn_clockwise(20252025).unwrap());

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p3.txt");
    let dial = RangeDial::build_from_ranges(&lines);
    println!("part 3 = {}", dial.turn_clockwise(202520252025).unwrap());
}

#[derive(Debug, Clone)]
struct RangeDial {
    ranges: Vec<RangeDir>,
    size: usize,
}

#[derive(Debug, Clone)]
enum RangeDir {
    Forward(Range<usize>),
    Reverse(Range<usize>),
}

impl RangeDir {
    fn count(&self) -> usize {
        match self {
            RangeDir::Forward(range) => range.end - range.start + 1,
            RangeDir::Reverse(range) => range.end - range.start + 1,
        }
    }

    fn get(&self, remaining_turns: usize) -> usize {
        match self {
            RangeDir::Forward(range) => remaining_turns + range.start,
            RangeDir::Reverse(range) => range.end - remaining_turns,
        }
    }
}

impl RangeDial {
    fn build_from_ranges<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut insert_back = true;
        let mut ranges = VecDeque::from([RangeDir::Forward(1..1)]);
        let mut size = 1;
        let mut inserted_in_front = 0;
        for range in lines {
            let (start, end) = range.as_ref().split_once('-').unwrap();
            let (start, end) = (
                start.parse::<usize>().unwrap(),
                end.parse::<usize>().unwrap(),
            );
            size += (end - start) + 1;
            if insert_back {
                ranges.push_back(RangeDir::Forward(start..end));
            } else {
                ranges.push_front(RangeDir::Reverse(start..end));
                inserted_in_front += 1;
            }
            insert_back = !insert_back;
        }

        ranges.rotate_left(inserted_in_front);
        ranges.make_contiguous();

        Self {
            ranges: ranges.as_slices().0.into(),
            size,
        }
    }

    fn turn_clockwise(&self, turns: usize) -> Option<usize> {
        let mut remaining_turns = turns % self.size;
        for entry in &self.ranges {
            if remaining_turns > entry.count() {
                remaining_turns -= entry.count();
            } else {
                return Some(entry.get(remaining_turns));
            }
        }

        None
    }
}

struct Dial {
    numbers: Vec<usize>,
}

impl Dial {
    fn build_from<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut insert_back = true;
        let mut numbers = VecDeque::from([1]);
        let mut inserted_in_front = 0;
        for number in lines {
            let number = number.as_ref().parse().unwrap();
            if insert_back {
                numbers.push_back(number);
            } else {
                numbers.push_front(number);
                inserted_in_front += 1;
            }
            insert_back = !insert_back;
        }

        numbers.rotate_left(inserted_in_front);
        numbers.make_contiguous();

        Self {
            numbers: numbers.as_slices().0.into(),
        }
    }

    // this works, but is slow for big ranges of numbers
    fn _build_from_ranges<S: AsRef<str>>(lines: &[S]) -> Self {
        let mut insert_back = true;
        let mut numbers = VecDeque::from([1]);
        let mut inserted_in_front = 0;
        for range in lines {
            let (start, end) = range.as_ref().split_once('-').unwrap();
            let (start, end) = (start.parse().unwrap(), end.parse().unwrap());
            if insert_back {
                for num in start..=end {
                    numbers.push_back(num);
                }
            } else {
                for num in start..=end {
                    numbers.push_front(num);
                    inserted_in_front += 1;
                }
            }
            insert_back = !insert_back;
        }

        numbers.rotate_left(inserted_in_front);
        numbers.make_contiguous();

        Self {
            numbers: numbers.as_slices().0.into(),
        }
    }

    fn turn_clockwise(&self, turns: usize) -> usize {
        self.numbers[turns % self.numbers.len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn text_example1() {
        let lines = ["72", "58", "47", "61", "67"];
        let dial = Dial::build_from(&lines);
        assert_eq!(67, dial.turn_clockwise(2025));
    }

    #[test]
    fn test_example2() {
        let lines = ["10-15", "12-13", "20-21", "19-23", "30-37"];
        let dial = Dial::_build_from_ranges(&lines);
        assert_eq!(
            vec![
                1, 10, 11, 12, 13, 14, 15, 20, 21, 30, 31, 32, 33, 34, 35, 36, 37, 23, 22, 21, 20,
                19, 13, 12
            ],
            dial.numbers
        );
    }
}
