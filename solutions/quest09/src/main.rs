use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p1.txt");
    println!("part 1 = {}", find_child_and_degree(&lines));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p2.txt");
    println!("part 2 = {}", find_all_children_and_degrees(&lines));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q09_p3.txt");
    let dragonducks: Vec<DragonDuck> = lines.iter().map(|line| line.parse().unwrap()).collect();
    let graph = build_edges(&dragonducks);
    println!("part 3 = {}", biggest_family(&graph));
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

fn build_edges(dragonducks: &[DragonDuck]) -> HashMap<usize, HashSet<usize>> {
    let mut graph = HashMap::<usize, HashSet<usize>>::new();
    for parent1 in 0..dragonducks.len() - 1 {
        for parent2 in parent1 + 1..dragonducks.len() {
            for child in 0..dragonducks.len() {
                if child == parent1 || child == parent2 {
                    continue;
                }
                if dragonducks[child].is_child_of(&dragonducks[parent1], &dragonducks[parent2]) {
                    let child = dragonducks[child].scale_number;
                    let parent1 = dragonducks[parent1].scale_number;
                    let parent2 = dragonducks[parent2].scale_number;
                    graph.entry(child).or_default().insert(parent1);
                    graph.entry(child).or_default().insert(parent2);
                    graph.entry(parent1).or_default().insert(child);
                    graph.entry(parent1).or_default().insert(parent2);
                    graph.entry(parent2).or_default().insert(child);
                    graph.entry(parent2).or_default().insert(parent1);
                }
            }
        }
    }
    graph
}

fn biggest_family(graph: &HashMap<usize, HashSet<usize>>) -> usize {
    let dragons = graph.keys().copied().collect::<Vec<_>>();
    let mut visited = HashSet::<usize>::new();
    let mut max_family_len = 0;
    let mut result = 0;
    for dragon in dragons {
        if visited.contains(&dragon) {
            continue;
        }
        let mut stack = Vec::from(&[dragon]);
        let mut family = Vec::new();
        while let Some(d) = stack.pop() {
            if visited.insert(d) {
                family.push(d);
                if let Some(neighbors) = graph.get(&d) {
                    for neighbor in neighbors {
                        stack.push(*neighbor);
                    }
                }
            }
        }
        if family.len() > max_family_len {
            result = family.iter().sum();
            max_family_len = family.len();
        }
    }

    result
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

struct DragonDuck {
    scale_number: usize,
    dna: Vec<char>,
}

impl DragonDuck {
    fn is_child_of(&self, parent1: &DragonDuck, parent2: &DragonDuck) -> bool {
        self.dna
            .iter()
            .enumerate()
            .all(|(idx, &ch)| ch == parent1.dna[idx] || ch == parent2.dna[idx])
    }
}

impl FromStr for DragonDuck {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (sn, dna) = s.split_once(':').unwrap();
        let scale_number = sn.parse().unwrap();
        let dna = dna.chars().collect();
        Ok(Self { scale_number, dna })
    }
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

    #[test]
    fn test_example3_1() {
        let lines = [
            "1:GCAGGCGAGTATGATACCCGGCTAGCCACCCC".to_string(),
            "2:TCTCGCGAGGATATTACTGGGCCAGACCCCCC".to_string(),
            "3:GGTGGAACATTCGAAAGTTGCATAGGGTGGTG".to_string(),
            "4:GCTCGCGAGTATATTACCGAACCAGCCCCTCA".to_string(),
            "5:GCAGCTTAGTATGACCGCCAAATCGCGACTCA".to_string(),
            "6:AGTGGAACCTTGGATAGTCTCATATAGCGGCA".to_string(),
            "7:GGCGTAATAATCGGATGCTGCAGAGGCTGCTG".to_string(),
        ];
        let dragonducks: Vec<DragonDuck> = lines.iter().map(|line| line.parse().unwrap()).collect();

        let graph = build_edges(&dragonducks);
        assert_eq!(12, biggest_family(&graph));
    }
}
