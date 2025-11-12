use std::collections::{HashMap, HashSet};

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q07_p1.txt");
    // let lines = aoclib::read_lines("input/test_1.txt");
    let names = lines[0].split(',').collect::<Vec<_>>();
    let mut mapping = HashMap::<char, HashSet<char>>::new();
    for map in &lines[1..] {
        let (letter, list) = map.split_once(" > ").unwrap();
        let list = list
            .split(',')
            .map(|ch| ch.chars().next().unwrap())
            .collect::<HashSet<char>>();
        mapping.insert(letter.chars().next().unwrap(), list);
    }

    for name in names {
        let mut cur_set: Option<HashSet<char>> = None;
        let mut success = true;
        for ch in name.chars() {
            if let Some(set) = cur_set
                && !set.contains(&ch)
            {
                success = false;
                break;
            }
            cur_set = mapping.get(&ch).cloned();
        }
        if success {
            println!("part 1 = {name}");
            break;
        }
    }
}
