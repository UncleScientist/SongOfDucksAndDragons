fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p1.txt");
    println!("part 1 = {}", count_swordfighting(&lines[0]));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        assert_eq!(5, count_swordfighting("AaAaa"));
    }
}
