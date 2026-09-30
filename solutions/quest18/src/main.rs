use std::{convert::Infallible, str::FromStr};

fn main() {
    let data = aoclib::read_text_records("input/everybody_codes_e2025_q18_p1.txt");
    let garden = Garden::new(&data);
    println!("Quest 18, part 1 = {}", garden.total_energy());
}

#[derive(Debug)]
struct Garden {
    plants: Vec<Plant>,
}

impl Garden {
    fn new(lines: &[String]) -> Self {
        Self {
            plants: lines.iter().map(|line| line.parse().unwrap()).collect(),
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

    fn total_energy(&self) -> usize {
        let last_plant_id = self.last_plant_id();
        self.energy_by_plant_id(last_plant_id)
    }

    fn energy_by_plant_id(&self, plant_id: usize) -> usize {
        let mut total = 0;
        for branch in &self.plants[plant_id - 1].branches {
            total += match branch {
                Branch::Free => 1,
                Branch::Connected {
                    plant_id,
                    thickness,
                } => *thickness * self.energy_by_plant_id(*plant_id),
            };
        }
        if total < self.plants[plant_id - 1].thickness {
            0
        } else {
            total
        }
    }
}

#[derive(Debug)]
struct Plant {
    thickness: usize,
    branches: Vec<Branch>,
}

#[derive(Debug)]
enum Branch {
    Free,
    Connected { plant_id: usize, thickness: usize },
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
            } else {
                let plant_id = words[4].parse().unwrap();
                let thickness = words[7].parse().unwrap();
                branches.push(Branch::Connected {
                    plant_id,
                    thickness,
                });
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
        assert_eq!(774, garden.total_energy());
    }
}
