fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q11_p1.txt");
    let columns = lines
        .iter()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<_>>();
    let mut flock = Flock::new(&columns);
    for _ in 0..10 {
        flock.round();
    }
    println!("part 1 = {}", flock.checksum());
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

    fn round(&mut self) {
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
                    self.phase = Phase::Done
                }
            }
            Phase::Done => {
                println!("done!");
            }
        }
    }

    fn checksum(&self) -> usize {
        self.columns
            .iter()
            .enumerate()
            .fold(0, |sum, (index, count)| sum + (index + 1) * *count)
    }
}

#[derive(Debug)]
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
        for _ in 0..10 {
            flock.round()
        }
        assert_eq!(109, flock.checksum());
    }
}
