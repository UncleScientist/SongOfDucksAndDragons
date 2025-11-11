fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p1.txt");
    println!("part 1 = {}", count_swordfighting(&lines[0]));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p2.txt");
    println!("part 1 = {}", count_all(&lines[0]));
}

fn count_swordfighting<S: AsRef<str>>(notes: S) -> usize {
    let mut knight_count = 0;
    let mut novice_count = 0;
    for c in notes.as_ref().chars() {
        match c {
            'A' => knight_count += 1,
            'a' => novice_count += knight_count,
            _ => {}
        }
    }

    novice_count
}

fn count_all<S: AsRef<str>>(notes: S) -> usize {
    let mut knight_count = [0usize; 3];
    let mut novice_count = [0usize; 3];
    for c in notes.as_ref().chars() {
        match c {
            'A' | 'B' | 'C' => {
                let idx = c as u8 - b'A';
                knight_count[idx as usize] += 1;
            }
            'a' | 'b' | 'c' => {
                let idx = c as u8 - b'a';
                novice_count[idx as usize] += knight_count[idx as usize];
            }
            _ => {
                panic!("invalid char '{c}'");
            }
        }
    }
    novice_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(5, count_swordfighting("AaAaa"));
    }

    #[test]
    fn test_example2() {
        assert_eq!(11, count_all("ABabACacBCbca"));
    }
}
