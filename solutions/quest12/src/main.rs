use std::collections::HashSet;

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q12_p1.txt");
    let grid = Grid::new(&lines);
    println!("part 1 = {}", grid.explode(&[(0, 0)]).len());

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q12_p2.txt");
    let grid = Grid::new(&lines);
    println!(
        "part 2 = {}",
        grid.explode(&[(0, 0), (grid.height - 1, grid.width - 1)])
            .len()
    );

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q12_p3.txt");
    let grid = Grid::new(&lines);
    println!("part 3 = {}", grid.triple_explosion());
}

type BarrelSet = HashSet<(isize, isize)>;

#[derive(Debug, Clone)]
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

    fn explode(&self, start: &[(isize, isize)]) -> BarrelSet {
        let mut stack = Vec::from(start);
        let mut visited = BarrelSet::new();

        while let Some(pos) = stack.pop() {
            let cur_value = self.barrels[pos.0 as usize][pos.1 as usize];
            if cur_value == 10 {
                continue;
            }

            if visited.insert(pos) {
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

        visited
    }

    fn triple_explosion(&self) -> usize {
        let mut cur_grid = self.clone();
        let mut total = 0;

        for _ in 0..3 {
            let mut most = BarrelSet::new();
            for row in 0..cur_grid.height {
                for col in 0..cur_grid.width {
                    let cur = cur_grid.explode(&[(row, col)]);

                    if cur.len() > most.len() {
                        most = cur;
                    }
                }
            }

            total += most.len();
            for barrel in most {
                cur_grid.barrels[barrel.0 as usize][barrel.1 as usize] = 10;
            }
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let lines = ["989601", "857782", "746543", "766789"];
        let grid = Grid::new(&lines);
        assert_eq!(16, grid.explode(&[(0, 0)]).len());
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
                .len()
        );
    }

    #[test]
    fn test_example3_1() {
        let lines = ["5411", "3362", "5235", "3112"];
        let grid = Grid::new(&lines);
        assert_eq!(14, grid.triple_explosion());
    }

    #[test]
    fn test_example3_2() {
        let lines = [
            "41951111131882511179",
            "32112222211518122215",
            "31223333322115122219",
            "31234444432147511128",
            "91223333322176121892",
            "61112222211166431583",
            "14661111166111111746",
            "11111119142122222177",
            "41222118881233333219",
            "71222127839122222196",
            "56111126279711111517",
        ];
        let grid = Grid::new(&lines);
        assert_eq!(136, grid.triple_explosion());
    }
}
