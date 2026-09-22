use std::{convert::Infallible, str::FromStr};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p1.txt");
    let wall = lines[0].parse::<Wall>().unwrap();
    println!("Quest 16, Part 1 = {}", wall.blocks_needed_for_length(90));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p2.txt");
    let spell = lines[0].parse::<Spell>().unwrap();
    println!("Quest 16, Part 2 = {}", spell.find_spell_for_wall());
}

#[derive(Debug)]
struct Wall {
    spell: Vec<usize>,
}

impl Wall {
    fn blocks_needed_for_length(&self, length: usize) -> usize {
        self.spell.iter().map(|num| length / num).sum()
    }
}

impl FromStr for Wall {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            spell: line.split(',').map(|txt| txt.parse().unwrap()).collect(),
        })
    }
}

#[derive(Debug)]
struct Spell {
    wall: Vec<usize>,
}

impl Spell {
    fn find_spell_for_wall(&self) -> usize {
        let mut entries = Vec::new();
        let mut working_list = self.wall.clone();
        while let Some(index) = working_list.iter().position(|num| *num == 1) {
            entries.push(index + 1);
            let mut point = index;
            while point < working_list.len() {
                working_list[point] -= 1;
                point += index + 1;
            }
        }

        entries.iter().product()
    }
}

impl FromStr for Spell {
    type Err = Infallible;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            wall: line.split(',').map(|txt| txt.parse().unwrap()).collect(),
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

    #[test]
    fn test_part_2() {
        let spell = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"
            .parse::<Spell>()
            .unwrap();
        assert_eq!(270, spell.find_spell_for_wall());
    }
}
