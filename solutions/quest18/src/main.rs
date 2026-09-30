use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p1.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 1 = {}", garden.total_energy());

    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p2.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 2 = {}", garden.total_energy());
}

#[derive(Debug)]
struct Garden {
    plants: Vec<Plant>,
    test_cases: Vec<Vec<isize>>,
}

impl Garden {
    fn new(lines: &[String]) -> Self {
        let has_test_cases =
            lines[lines.len() - 1].starts_with("\n0") || lines[lines.len() - 1].starts_with("\n1");

        let (test_cases, limit) = if has_test_cases {
            (
                Self::extract_test_cases(&lines[lines.len() - 1]),
                lines.len() - 1,
            )
        } else {
            (vec![vec![1isize; lines.len()]], lines.len())
        };

        Self {
            plants: lines[0..limit]
                .iter()
                .map(|line| line.parse().unwrap())
                .collect(),
            test_cases,
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
        self.energy_by_plant_id(last_plant_id, test_case)
    }

    fn energy_by_plant_id(&self, plant_id: usize, test_case: usize) -> isize {
        let mut total = 0;
        for branch in &self.plants[plant_id - 1].branches {
            total += match branch {
                Branch::Free => self.test_cases[test_case][plant_id - 1],
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

    fn extract_test_cases(lines: &str) -> Vec<Vec<isize>> {
        lines
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.split(' ')
                    .map(|digit| digit.parse().unwrap())
                    .collect()
            })
            .collect()
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
}
