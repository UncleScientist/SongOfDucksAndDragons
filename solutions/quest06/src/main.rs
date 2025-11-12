fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p1.txt");
    println!("part 1 = {}", count_swordfighting(&lines[0]));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p2.txt");
    println!("part 2 = {}", count_all(&lines[0]));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q06_p3.txt");
    println!("part 2 = {}", count_final(&lines[0], 1000, 1000));
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

fn count_final<S: AsRef<str>>(notes: S, repeat: usize, dist: usize) -> usize {
    let notes = notes.as_ref().chars().collect::<Vec<_>>();
    let mut novice_count = [0usize; 3];
    for (pos, c) in notes.repeat(repeat).iter().enumerate() {
        if *c >= 'A' && *c <= 'C' {
            continue;
        }
        let start = pos.saturating_sub(dist);
        let end = (pos + dist).min(notes.len() * repeat - 1);
        let idx = (*c as u8 - b'a') as usize;
        let kval = c.to_ascii_uppercase();
        for knight in start..=end {
            if notes[knight % notes.len()] == kval {
                novice_count[idx] += 1;
            }
        }
    }
    novice_count.iter().sum()
}

fn _count_final<S: AsRef<str>>(notes: S, repeat: usize, dist: usize) -> usize {
    let mut knight_count = [const { Vec::<usize>::new() }; 3];
    for (pos, c) in notes.as_ref().repeat(repeat).chars().enumerate() {
        if c >= 'A' && c <= 'C' {
            let idx = (c as u8 - b'A') as usize;
            knight_count[idx].push(pos);
        }
    }
    println!("knights:");
    for k in &knight_count {
        println!("  {k:?}");
    }

    let mut novice_count = [0usize; 3];
    for (pos, c) in notes.as_ref().repeat(repeat).chars().enumerate() {
        if c >= 'a' && c <= 'c' {
            println!("-");
            let idx = (c as u8 - b'a') as usize;
            for knight in &knight_count[idx] {
                if knight.abs_diff(pos) <= dist {
                    println!("{c} @ {pos}: knight at {knight}");
                    novice_count[idx] += 1;
                }
            }
        }
    }
    println!("{novice_count:?}");
    novice_count.iter().sum::<usize>()
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

    #[test]
    fn test_example_3_1() {
        assert_eq!(34, _count_final("AABCBABCABCabcabcABCCBAACBCa", 1, 10));
    }

    #[test]
    fn test_example3_2() {
        _count_final("AABCBABCABCabcabcABCCBAACBCa", 2, 10);
        assert_eq!(72, count_final("AABCBABCABCabcabcABCCBAACBCa", 2, 10));
    }

    #[test]
    fn test_example3_3() {
        _count_final("AABCBABCABCabcabcABCCBAACBCa", 1000, 1000);
        assert_eq!(
            3442321,
            count_final("AABCBABCABCabcabcABCCBAACBCa", 1000, 1000)
        );
    }
}
