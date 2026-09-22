use std::{convert::Infallible, str::FromStr};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p1.txt");
    let wall = lines[0].parse::<Wall>().unwrap();
    println!("Quest 16, Part 1 = {}", wall.blocks_needed_for_length(90));
}

#[derive(Debug)]
struct Wall {
    pattern: Vec<usize>,
}

impl Wall {
    fn blocks_needed_for_length(&self, length: usize) -> usize {
        self.pattern.iter().map(|num| length / num).sum()
    }
}

impl FromStr for Wall {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            pattern: line.split(',').map(|txt| txt.parse().unwrap()).collect(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let wall = "1,2,3,5,9".parse::<Wall>().unwrap();
        assert_eq!(193, wall.blocks_needed_for_length(90));
    }
}
