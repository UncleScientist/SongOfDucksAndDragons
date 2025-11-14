fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p1.txt");
    println!("part 1 = {}", find_child_and_degree(&lines));
}

fn find_child_and_degree(lines: &[String]) -> usize {
    let lines = lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for parent1 in 0..lines.len() - 1 {
        for parent2 in parent1 + 1..lines.len() {
            for child in 0..lines.len() {
                if parent1 == child || parent2 == child {
                    continue;
                }
                if lines[child]
                    .iter()
                    .enumerate()
                    .all(|(idx, &ch)| ch == lines[parent1][idx] || ch == lines[parent2][idx])
                {
                    let p1 = lines[parent1]
                        .iter()
                        .zip(&lines[child])
                        .filter(|(p1, c)| **p1 == **c)
                        .count();
                    let p2 = lines[parent2]
                        .iter()
                        .zip(&lines[child])
                        .filter(|(p2, c)| **p2 == **c)
                        .count();
                    return p1 * p2;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let lines = vec![
            "1:CAAGCGCTAAGTTCGCTGGATGTGTGCCCGCG".to_string(),
            "2:CTTGAATTGGGCCGTTTACCTGGTTTAACCAT".to_string(),
            "3:CTAGCGCTGAGCTGGCTGCCTGGTTGACCGCG".to_string(),
        ];
        assert_eq!(414, find_child_and_degree(&lines));
    }
}
