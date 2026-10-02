use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p1.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 1 = {}", garden.total_energy());

    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p2.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 2 = {}", garden.total_energy());

    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p3.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 3 = {}", garden.computed_energy_difference());
}

#[derive(Debug)]
struct Garden {
    plants: Vec<Plant>,
    _free_plant_count: usize,
    test_cases: Vec<u128>,
    positive_case: u128,
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
            vec![u128::MAX]
        };

        let mut positive_case: u128 = 0;
        for plant in &plants {
            for branch in &plant.branches {
                match branch {
                    Branch::Free => {}
                    Branch::Connected {
                        plant_id,
                        thickness,
                    } => {
                        if *plant_id <= _free_plant_count && *thickness > 0 {
                            positive_case |= 1 << (plant_id - 1);
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
        self.energy_by_plant_id(last_plant_id, self.test_cases[test_case])
    }

    fn energy_by_plant_id(&self, plant_id: usize, test_case: u128) -> isize {
        let mut total = 0;
        for branch in &self.plants[plant_id - 1].branches {
            total += match branch {
                Branch::Free => {
                    if test_case & (1 << (plant_id - 1)) != 0 {
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

    fn extract_test_cases(lines: &str) -> Vec<u128> {
        lines
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(' ').enumerate().fold(0, |case, (idx, digit)| {
                    if digit == "1" {
                        case | (1 << idx)
                    } else {
                        case
                    }
                })
            })
            .collect()
    }

    fn computed_energy_difference(&self) -> isize {
        let last_plant_id = self.last_plant_id();

        let max_energy = self.energy_by_plant_id(last_plant_id, self.positive_case);
        let mut total_diff = 0;

        for case in 0..self.test_cases.len() {
            let energy = self.energy_by_plant_id(last_plant_id, self.test_cases[case]);
            if energy > 0 {
                total_diff += max_energy - energy;
            }
        }

        total_diff
    }

    fn _naive_energy_difference(&self) -> isize {
        let last_plant_id = self.last_plant_id();
        let width = self._free_plant_count;

        for case in 0..self.test_cases.len() {
            let test_case = self.test_cases[case];
            let energy = self.energy_by_plant_id(last_plant_id, self.test_cases[case]);
            println!("{test_case:0width$b} | {energy}");
        }

        let mut case = 1u128;
        while case < 2u128.pow(self._free_plant_count as u32) {
            let energy = self.energy_by_plant_id(last_plant_id, case);
            println!("{case:0width$b} | {energy}");
            case <<= 1;
        }

        let mut max_energy = isize::MIN;
        for case in 0u128..2u128.pow(self._free_plant_count as u32) {
            max_energy = max_energy.max(self.energy_by_plant_id(last_plant_id, case));
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
        let total_diff = garden._naive_energy_difference();
        assert_eq!(946, total_diff);
    }
}
