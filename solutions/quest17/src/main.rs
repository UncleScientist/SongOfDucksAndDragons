fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p1.txt");
    let volcano = Volcano::new(&lines);
    println!("Quest 17, Part 1 = {}", volcano.total_sum_at_radius(10));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p2.txt");
    let volcano = Volcano::new(&lines);
    println!("Quest 17, Part 2 = {}", volcano.max_destruction_product());
}

#[derive(Debug)]
struct Volcano {
    grid: Vec<Vec<u8>>,
    source: (usize, usize), // row, col
}

impl Volcano {
    fn new(lines: &[String]) -> Self {
        let mut grid = Vec::new();
        let mut source = (0, 0);

        for line in lines {
            let mut grid_row = Vec::new();
            for ch in line.chars() {
                if ch == '@' {
                    source = (grid.len(), grid_row.len());
                    grid_row.push(0);
                } else {
                    grid_row.push(ch as u8 - b'0');
                }
            }
            grid.push(grid_row);
        }

        Volcano { grid, source }
    }

    fn total_sum_at_radius(&self, radius: usize) -> usize {
        let mut total = 0;
        for (row, line) in self.grid.iter().enumerate() {
            for (col, amount) in line.iter().enumerate() {
                if self.within(row, col, radius) {
                    total += *amount as usize;
                }
            }
        }
        total
    }

    fn sum_at_radius(&self, radius: usize) -> usize {
        self.total_sum_at_radius(radius) - self.total_sum_at_radius(radius - 1)
    }

    fn within(&self, row: usize, col: usize, radius: usize) -> bool {
        row.abs_diff(self.source.0).pow(2) + col.abs_diff(self.source.1).pow(2) <= radius * radius
    }

    fn max_destruction_product(&self) -> usize {
        let max_radius = self.grid[0].len() - self.source.0;
        let (radius, amount) = (1..max_radius)
            .map(|radius| (radius, self.sum_at_radius(radius)))
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        radius * amount
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let lines = aoclib::read_lines("test-input/part1");
        let volcano = Volcano::new(&lines);
        assert_eq!(1573, volcano.total_sum_at_radius(10));
    }

    #[test]
    fn test_part_2() {
        let lines = aoclib::read_lines("test-input/part2");
        let volcano = Volcano::new(&lines);
        assert_eq!(26, volcano.sum_at_radius(1));
        assert_eq!(49, volcano.sum_at_radius(2));
        assert_eq!(109, volcano.sum_at_radius(3));
        assert_eq!(146, volcano.sum_at_radius(4));
        assert_eq!(218, volcano.sum_at_radius(5));
        assert_eq!(199, volcano.sum_at_radius(6));
    }

    #[test]
    fn test_part_2_max() {
        let lines = aoclib::read_lines("test-input/part2");
        let volcano = Volcano::new(&lines);
        assert_eq!(1090, volcano.max_destruction_product());
    }
}
