use std::{convert::Infallible, fmt::Binary, str::FromStr};

use macroquad::prelude::*;

#[macroquad::main("When Roots Remember")]
async fn main() {
    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p1.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 1 = {}", garden.total_energy());

    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p2.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 2 = {}", garden.total_energy());

    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p3.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 3 = {}", garden.computed_energy_difference());

    loop {
        clear_background(BLACK);

        for (idx, case) in garden.test_cases.iter().enumerate() {
            case.draw(idx / 10, idx % 10);
        }

        next_frame().await;
    }
}

#[derive(Debug)]
struct Garden {
    plants: Vec<Plant>,
    _free_plant_count: usize,
    test_cases: Vec<TestCase>,
    positive_case: TestCase,
}

impl Garden {
    fn new(lines: &[String]) -> Self {
        let has_test_cases =
            lines[lines.len() - 1].starts_with("\n0") || lines[lines.len() - 1].starts_with("\n1");

        let limit = if has_test_cases {
            lines.len() - 1
        } else {
            lines.len()
        };

        let plants = lines[0..limit]
            .iter()
            .map(|line| line.parse().unwrap())
            .collect::<Vec<Plant>>();

        let _free_plant_count = plants
            .iter()
            .filter(|plant| plant.branches.len() == 1 && matches!(plant.branches[0], Branch::Free))
            .count();

        let test_cases = if has_test_cases {
            Self::extract_test_cases(&lines[limit])
        } else {
            vec![TestCase::all()]
        };

        let mut positive_case = TestCase::new();
        for plant in &plants {
            for branch in &plant.branches {
                match branch {
                    Branch::Free => {}
                    Branch::Connected {
                        plant_id,
                        thickness,
                    } => {
                        if *plant_id <= _free_plant_count && *thickness > 0 {
                            positive_case.set_bit(plant_id - 1);
                        }
                    }
                }
            }
        }

        Self {
            plants,
            _free_plant_count,
            test_cases,
            positive_case,
        }
    }

    fn last_plant_id(&self) -> usize {
        let mut mentioned = vec![false; self.plants.len()];
        for plant in &self.plants {
            for branch in &plant.branches {
                if let Branch::Connected { plant_id, .. } = branch {
                    mentioned[*plant_id - 1] = true;
                }
            }
        }
        mentioned.iter().position(|p| !*p).unwrap() + 1
    }

    fn total_energy(&self) -> isize {
        (0..self.test_cases.len())
            .map(|case| self.energy_for_test_case(case))
            .sum()
    }

    fn energy_for_test_case(&self, test_case: usize) -> isize {
        let last_plant_id = self.last_plant_id();
        self.energy_by_plant_id(last_plant_id, &self.test_cases[test_case])
    }

    fn energy_by_plant_id(&self, plant_id: usize, test_case: &TestCase) -> isize {
        let mut total = 0;
        for branch in &self.plants[plant_id - 1].branches {
            total += match branch {
                Branch::Free => {
                    if test_case.is_bit_set(plant_id - 1) {
                        1
                    } else {
                        0
                    }
                }
                Branch::Connected {
                    plant_id,
                    thickness,
                } => *thickness * self.energy_by_plant_id(*plant_id, test_case),
            };
        }
        if total < self.plants[plant_id - 1].thickness {
            0
        } else {
            total
        }
    }

    fn extract_test_cases(lines: &str) -> Vec<TestCase> {
        lines
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect()
    }

    fn computed_energy_difference(&self) -> isize {
        let last_plant_id = self.last_plant_id();

        let max_energy = self.energy_by_plant_id(last_plant_id, &self.positive_case);
        let mut total_diff = 0;

        for case in 0..self.test_cases.len() {
            let energy = self.energy_by_plant_id(last_plant_id, &self.test_cases[case]);
            if energy > 0 {
                total_diff += max_energy - energy;
            }
        }

        total_diff
    }

    #[cfg(test)]
    fn naive_energy_difference(&self) -> isize {
        let last_plant_id = self.last_plant_id();
        let width = self._free_plant_count;

        for case in 0..self.test_cases.len() {
            let test_case = &self.test_cases[case];
            let energy = self.energy_by_plant_id(last_plant_id, &self.test_cases[case]);
            println!("{test_case:0width$b} | {energy}");
        }

        for case in TestCase::pow2range(0, self._free_plant_count as u32) {
            let energy = self.energy_by_plant_id(last_plant_id, &case);
            println!("{case:0width$b} | {energy}");
        }

        let mut max_energy = isize::MIN;
        for case in TestCase::pow2range(0, self._free_plant_count as u32) {
            max_energy = max_energy.max(self.energy_by_plant_id(last_plant_id, &case));
        }

        let mut total_diff = 0;
        for case in 0..self.test_cases.len() {
            let energy = self.energy_for_test_case(case);
            if energy > 0 {
                total_diff += max_energy - energy;
            }
        }

        total_diff
    }
}

#[derive(Debug)]
struct Plant {
    thickness: isize,
    branches: Vec<Branch>,
}

#[derive(Debug)]
enum Branch {
    Free,
    Connected { plant_id: usize, thickness: isize },
}

impl FromStr for Plant {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut thickness = 0;
        let mut branches = Vec::new();
        for line in line.split('\n') {
            if line.is_empty() {
                continue;
            }
            let words = line.split(' ').collect::<Vec<_>>();
            if words[0] == "Plant" {
                let thickstr = &words[4][0..words[4].len() - 1];
                thickness = thickstr.parse().unwrap();
            } else if words[1] == "free" {
                branches.push(Branch::Free)
            } else if words[1] == "branch" {
                let plant_id = words[4].parse().unwrap();
                let thickness = words[7].parse().unwrap();
                branches.push(Branch::Connected {
                    plant_id,
                    thickness,
                });
            } else {
                panic!("invalid plant");
            }
        }
        Ok(Plant {
            thickness,
            branches,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct TestCase(u128);

impl Binary for TestCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl TestCase {
    fn new() -> Self {
        Self(0)
    }

    fn all() -> Self {
        Self(u128::MAX)
    }

    fn is_bit_set(&self, bit: usize) -> bool {
        (self.0 & (1 << bit)) != 0
    }

    fn set_bit(&mut self, bit: usize) {
        self.0 |= 1 << bit;
    }

    #[cfg(test)]
    fn iter_bits() -> TestCaseIterator {
        TestCaseIterator {
            iter_type: TestCaseIteratorType::Shift,
            next_bit: 1,
            bits: 0,
            max: 0,
        }
    }

    fn iter_set_bits(&self) -> TestCaseIterator {
        TestCaseIterator {
            iter_type: TestCaseIteratorType::SetBit,
            next_bit: 0,
            bits: self.0,
            #[cfg(test)]
            max: 0,
        }
    }

    #[cfg(test)]
    fn pow2range(start: u128, p2end: u32) -> TestCaseIterator {
        TestCaseIterator {
            iter_type: TestCaseIteratorType::Increment,
            next_bit: start,
            bits: 0,
            max: 2u128.pow(p2end),
        }
    }

    fn draw(&self, row: usize, col: usize) {
        const SIZE: f32 = 5.0;

        let (row, col) = ((row * 10) as f32, (col * 10) as f32);

        for case in self.iter_set_bits() {
            let num = case.0;
            let grid_row = (num % 9) as f32;
            let grid_col = (num / 9) as f32;

            draw_rectangle(
                SIZE + (row + grid_row) * SIZE,
                SIZE + (col + grid_col) * SIZE,
                SIZE,
                SIZE,
                GREEN,
            );
        }
    }
}

impl FromStr for TestCase {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(line
            .split(' ')
            .enumerate()
            .fold(TestCase(0), |case, (idx, digit)| {
                if digit == "1" {
                    TestCase(case.0 | (1 << idx))
                } else {
                    TestCase(case.0)
                }
            }))
    }
}

#[derive(Debug)]
enum TestCaseIteratorType {
    #[cfg(test)]
    Increment,
    #[cfg(test)]
    Shift,
    SetBit,
}

#[derive(Debug)]
struct TestCaseIterator {
    iter_type: TestCaseIteratorType,
    bits: u128,
    next_bit: u128,
    #[cfg(test)]
    max: u128,
}

impl Iterator for TestCaseIterator {
    type Item = TestCase;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter_type {
            #[cfg(test)]
            TestCaseIteratorType::Shift => {
                if self.next_bit == 0 {
                    None
                } else {
                    let return_value = Some(TestCase(self.next_bit));
                    self.next_bit <<= 1;
                    return_value
                }
            }
            #[cfg(test)]
            TestCaseIteratorType::Increment => {
                if self.next_bit >= self.max {
                    None
                } else {
                    let return_value = Some(TestCase(self.next_bit));
                    self.next_bit += 1;
                    return_value
                }
            }
            TestCaseIteratorType::SetBit => {
                if self.next_bit >= 128 || (1 << self.next_bit) > self.bits {
                    None
                } else {
                    while (1 << self.next_bit) <= self.bits {
                        if (1 << self.next_bit) & self.bits != 0 {
                            let return_value = Some(TestCase(self.next_bit));
                            self.next_bit += 1;
                            return return_value;
                        }
                        self.next_bit += 1;
                    }
                    None
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_last_plant() {
        let data = aoclib::read_text_records("test-input/part1");
        let garden = Garden::new(&data);
        assert_eq!(7, garden.last_plant_id());
    }

    #[test]
    fn test_part_1() {
        let data = aoclib::read_text_records("test-input/part1");
        let garden = Garden::new(&data);
        assert_eq!(774, garden.energy_for_test_case(0));
    }

    #[test]
    fn test_part_2() {
        let data = aoclib::read_text_records("test-input/part2");
        let garden = Garden::new(&data);
        assert_eq!(324, garden.total_energy());
    }

    #[test]
    fn test_part_3() {
        let data = aoclib::read_text_records("test-input/part3");
        let garden = Garden::new(&data);
        let total_diff = garden.naive_energy_difference();
        assert_eq!(946, total_diff);
    }

    #[test]
    fn test_shift_bits() {
        let mut iter = TestCase::iter_bits();
        let next = iter.next();
        assert_eq!(next, Some(TestCase(1 << 0)));
        let next = iter.next();
        assert_eq!(next, Some(TestCase(1 << 1)));
        let next = iter.next();
        assert_eq!(next, Some(TestCase(1 << 2)));
        let next = iter.next();
        assert_eq!(next, Some(TestCase(1 << 3)));
    }

    #[test]
    fn test_iterator() {
        let num: u128 =
            0b101010100000111000110000010101000001100001001100111001010010010010000010001111100;
        let case = TestCase(num);
        let mut iter = case.iter_set_bits();
        let first_bit = iter.next();
        assert_eq!(Some(TestCase(2)), first_bit);
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        let next_bit = iter.next();
        assert_eq!(Some(TestCase(10)), next_bit);
    }
}
