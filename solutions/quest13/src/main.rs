use std::collections::VecDeque;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p1.txt");
    let dial = Dial::build_from(&lines);
    println!("part 1 = {}", dial.turn_clockwise(2025));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p2.txt");
    let dial = Dial::build_from_ranges(&lines);
    println!("part 2 = {}", dial.turn_clockwise(20252025));
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

        Dial {
            numbers: numbers.as_slices().0.into(),
        }
    }

    fn build_from_ranges<S: AsRef<str>>(lines: &[S]) -> Self {
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

        Dial {
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
        let dial = Dial::build_from_ranges(&lines);
        assert_eq!(
            vec![
                1, 10, 11, 12, 13, 14, 15, 20, 21, 30, 31, 32, 33, 34, 35, 36, 37, 23, 22, 21, 20,
                19, 13, 12
            ],
            dial.numbers
        );
    }
}
