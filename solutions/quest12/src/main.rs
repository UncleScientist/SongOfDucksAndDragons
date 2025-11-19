use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q12_p1.txt");
    let grid = Grid::new(&lines);
    println!("part 1 = {}", grid.explode(&[(0, 0)]));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q12_p2.txt");
    let grid = Grid::new(&lines);
    println!(
        "part 2 = {}",
        grid.explode(&[(0, 0), (grid.height - 1, grid.width - 1)])
    );
}

#[derive(Debug)]
struct Grid {
    width: isize,
    height: isize,
    barrels: Vec<Vec<u8>>,
}

impl Grid {
    fn new<S: AsRef<str>>(lines: &[S]) -> Self {
        let width = lines[0].as_ref().len() as isize;
        let height = lines.len() as isize;
        let mut barrels = Vec::new();

        for line in lines.iter().as_ref() {
            barrels.push(line.as_ref().chars().map(|ch| ch as u8 - b'0').collect());
        }

        Self {
            width,
            height,
            barrels,
        }
    }

    fn explode(&self, start: &[(isize, isize)]) -> usize {
        let mut stack = Vec::from(start);
        let mut visited = HashSet::<(isize, isize)>::new();

        while let Some(pos) = stack.pop() {
            if visited.insert(pos) {
                let cur_value = self.barrels[pos.0 as usize][pos.1 as usize];
                for dir in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    if new_pos.0 < 0
                        || new_pos.1 < 0
                        || new_pos.0 >= self.height
                        || new_pos.1 >= self.width
                    {
                        continue;
                    }
                    let new_value = self.barrels[new_pos.0 as usize][new_pos.1 as usize];
                    if new_value <= cur_value {
                        stack.push(new_pos);
                    }
                }
            }
        }

        visited.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let lines = ["989601", "857782", "746543", "766789"];
        let grid = Grid::new(&lines);
        assert_eq!(16, grid.explode(&[(0, 0)]));
    }

    #[test]
    fn test_example2() {
        let lines = [
            "9589233445",
            "9679121695",
            "8469121876",
            "8352919876",
            "7342914327",
            "7234193437",
            "6789193538",
            "6781219648",
            "5691219769",
            "5443329859",
        ];
        let grid = Grid::new(&lines);
        assert_eq!(
            58,
            grid.explode(&[(0, 0), (grid.height - 1, grid.width - 1)])
        );
    }
}
