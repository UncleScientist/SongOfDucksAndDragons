fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p1.txt");
    println!("part 1 = {}", find_child_and_degree(&lines));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p2.txt");
    println!("part 1 = {}", find_all_children_and_degrees(&lines));
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
                if is_child_of(&lines[parent1], &lines[parent2], &lines[child]) {
                    return degree_of_similarity(&lines[parent1], &lines[parent2], &lines[child]);
                }
            }
        }
    }
    0
}

fn find_all_children_and_degrees(lines: &[String]) -> usize {
    let lines = lines
        .iter()
        .map(|line| line.split_once(':').unwrap().1.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut total_degree_of_similarity = 0;
    for child in 0..lines.len() {
        for parent1 in 0..lines.len() - 1 {
            for parent2 in parent1 + 1..lines.len() {
                if child == parent1 || child == parent2 {
                    continue;
                }
                if is_child_of(&lines[parent1], &lines[parent2], &lines[child]) {
                    total_degree_of_similarity +=
                        degree_of_similarity(&lines[parent1], &lines[parent2], &lines[child]);
                }
            }
        }
    }

    total_degree_of_similarity
}

fn is_child_of(parent1: &[char], parent2: &[char], child: &[char]) -> bool {
    child
        .iter()
        .enumerate()
        .all(|(idx, &ch)| ch == parent1[idx] || ch == parent2[idx])
}

fn degree_of_similarity(parent1: &[char], parent2: &[char], child: &[char]) -> usize {
    parent1
        .iter()
        .zip(child)
        .filter(|(p1, c)| **p1 == **c)
        .count()
        * parent2
            .iter()
            .zip(child)
            .filter(|(p2, c)| **p2 == **c)
            .count()
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

    #[test]
    fn test_example2() {
        let lines = vec![
            "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC".to_string(),
            "2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC".to_string(),
            "3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG".to_string(),
            "4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA".to_string(),
            "5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA".to_string(),
            "6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA".to_string(),
            "7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG".to_string(),
        ];
        assert_eq!(1245, find_all_children_and_degrees(&lines));
    }
}
