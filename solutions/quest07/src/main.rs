use std::{collections::{HashMap, HashSet}, path::Path};

fn main() {
    let Rules {names, mapping} = Rules::read("input/everybody_codes_e2025_q07_p1.txt");
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

    let Rules {names, mapping} = Rules::read("input/everybody_codes_e2025_q07_p2.txt");
    let mut total = 0;
    for (idx, name) in names.into_iter().enumerate() {
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
            total += idx + 1;
        }
    }
    println!("part 2 = {total}");
}

struct Rules {
    names: Vec<String>,
    mapping: HashMap<char, HashSet<char>>,
}

impl Rules {
    fn read<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
    let names = lines[0].split(',').map(|name| name.to_string()).collect::<Vec<_>>();
    let mut mapping = HashMap::<char, HashSet<char>>::new();
    for map in &lines[1..] {
        let (letter, list) = map.split_once(" > ").unwrap();
        let list = list
            .split(',')
            .map(|ch| ch.chars().next().unwrap())
            .collect::<HashSet<char>>();
        mapping.insert(letter.chars().next().unwrap(), list);
    }
    Self { names, mapping }
    }
}

