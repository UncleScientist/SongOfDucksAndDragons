use std::collections::VecDeque;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q13_p1.txt");
    let dial = Dial::build_from(&lines);
    println!("part 1 = {}", dial.turn_clockwise(2025));
}

struct Dial {
    numbers: VecDeque<usize>,
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

        Dial { numbers }
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
}
