fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q17_p1.txt");
    let volcano = Volcano::new(&lines);
    println!("Quest 17, Part 1 = {}", volcano.sum_with_radius(10));
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

    fn sum_with_radius(&self, radius: usize) -> usize {
        let mut total = 0;
        for (row, line) in self.grid.iter().enumerate() {
            for (col, amount) in line.iter().enumerate() {
                if self.covered(row, col, radius) {
                    total += *amount as usize;
                }
            }
        }
        total
    }

    fn covered(&self, row: usize, col: usize, radius: usize) -> bool {
        row.abs_diff(self.source.0).pow(2) + col.abs_diff(self.source.1).pow(2) <= radius * radius
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = r#"189482189843433862719
279415473483436249988
432746714658787816631
428219317375373724944
938163982835287292238
627369424372196193484
539825864246487765271
517475755641128575965
685934212385479112825
815992793826881115341
1737798467@7983146242
867597735651751839244
868364647534879928345
519348954366296559425
134425275832833829382
764324337429656245499
654662236199275446914
317179356373398118618
542673939694417586329
987342622289291613318
971977649141188759131"#;

    #[test]
    fn test_part_1() {
        let lines = TEST_INPUT
            .split('\n')
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        let volcano = Volcano::new(&lines);
        assert_eq!(1573, volcano.sum_with_radius(10));
    }
}
