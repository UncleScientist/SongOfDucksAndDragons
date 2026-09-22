use std::{convert::Infallible, str::FromStr};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p1.txt");
    let wall = lines[0].parse::<Wall>().unwrap();
    println!("Quest 16, Part 1 = {}", wall.blocks_needed_for_length(90));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p2.txt");
    let spell = lines[0].parse::<Spell>().unwrap();
    println!(
        "Quest 16, Part 2 = {}",
        spell.find_spell_for_wall().iter().product::<usize>()
    );

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q16_p3.txt");
    let spell = lines[0].parse::<Spell>().unwrap();
    let wall: Wall = spell.into();
    println!(
        "Quest 16, Part 3 = {:?}",
        wall.length_needed_for_blocks(202520252025000)
    );
}

#[derive(Debug)]
struct Wall {
    spell: Vec<usize>,
}

impl Wall {
    fn blocks_needed_for_length(&self, length: usize) -> usize {
        self.spell.iter().map(|num| length / num).sum()
    }

    fn length_needed_for_blocks(&self, blocks: usize) -> usize {
        let mut low = 1;
        let mut high = usize::MAX / 2;
        while low < high {
            let midpoint = (low + high).div_ceil(2);
            let blocks_needed = self.blocks_needed_for_length(midpoint);
            if blocks_needed <= blocks {
                low = midpoint;
            } else {
                high = midpoint - 1;
            }
        }

        low
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
    fn find_spell_for_wall(&self) -> Vec<usize> {
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

        entries
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

impl From<Spell> for Wall {
    fn from(spell: Spell) -> Self {
        Wall {
            spell: spell.find_spell_for_wall(),
        }
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
        assert_eq!(270, spell.find_spell_for_wall().iter().product::<usize>());
    }

    #[test]
    fn test_part_3() {
        let spell = "1,2,2,2,2,3,1,2,3,3,1,3,1,2,3,2,1,4,1,3,2,2,1,3,2,2"
            .parse::<Spell>()
            .unwrap();
        let wall: Wall = spell.into();
        assert_eq!(
            94439495762954,
            wall.length_needed_for_blocks(202520252025000)
        );
    }
}
