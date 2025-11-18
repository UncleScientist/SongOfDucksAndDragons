fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q11_p1.txt");
    let columns = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    let mut flock = Flock::new(&columns);
    flock.do_rounds(10);
    println!("part 1 = {}", flock.checksum());

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q11_p2.txt");
    let columns = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    let mut flock = Flock::new(&columns);
    println!("part 2 = {}", flock.rounds_until_balanced());
}

#[derive(Debug)]
struct Flock {
    phase: Phase,
    columns: Vec<usize>,
}

impl Flock {
    fn new(columns: &[usize]) -> Self {
        Self {
            phase: Phase::First,
            columns: columns.to_vec(),
        }
    }

    fn round(&mut self) -> bool {
        match self.phase {
            Phase::First => {
                let mut moved = false;
                for idx in 0..self.columns.len() - 1 {
                    if self.columns[idx] > self.columns[idx + 1] {
                        self.columns[idx] -= 1;
                        self.columns[idx + 1] += 1;
                        moved = true;
                    }
                }
                if !moved {
                    self.phase = Phase::Second;
                    self.round();
                }
            }
            Phase::Second => {
                let mut moved = false;
                for idx in 0..self.columns.len() - 1 {
                    if self.columns[idx + 1] > self.columns[idx] {
                        self.columns[idx] += 1;
                        self.columns[idx + 1] -= 1;
                        moved = true;
                    }
                }
                if !moved {
                    self.phase = Phase::Done;
                    return true;
                }
            }
            Phase::Done => {
                println!("done!");
            }
        }

        false
    }

    fn do_rounds(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.round();
        }
    }

    fn rounds_until_balanced(&mut self) -> usize {
        let mut count = 0;
        while !self.round() {
            count += 1;
        }
        count
    }

    fn checksum(&self) -> usize {
        self.columns
            .iter()
            .enumerate()
            .fold(0, |sum, (index, count)| sum + (index + 1) * *count)
    }
}

#[derive(Debug, PartialEq)]
enum Phase {
    First,
    Second,
    Done,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let columns = [9, 1, 1, 4, 9, 6];
        let mut flock = Flock::new(&columns);
        flock.do_rounds(10);
        assert_eq!(109, flock.checksum());
    }

    #[test]
    fn test_example2_2() {
        let columns = [805, 706, 179, 48, 158, 150, 232, 885, 598, 524, 423];
        let mut flock = Flock::new(&columns);
        assert_eq!(1579, flock.rounds_until_balanced());
    }
}
