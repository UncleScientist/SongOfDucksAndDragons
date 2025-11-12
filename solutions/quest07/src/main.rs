use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

fn main() {
    let Rules { names, mapping } = Rules::read("input/everybody_codes_e2025_q07_p1.txt");
    for name in names {
        if valid_name(&name, &mapping) {
            println!("part 1 = {name}");
            break;
        }
    }

    let Rules { names, mapping } = Rules::read("input/everybody_codes_e2025_q07_p2.txt");
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

    let Rules {names, mapping} = Rules::read("input/everybody_codes_e2025_q07_p3.txt");
    // let Rules { names, mapping } = Rules::read("input/test_3.txt");

    let mut cache = Cache::default();
    for name in names.iter().filter(|name| valid_name(*name, &mapping)) {
        cache.find(name.to_string(), &mapping);
    }
    println!("part 3 = {}", cache.cache.len());
}

fn valid_name(name: &str, mapping: &HashMap<char, HashSet<char>>) -> bool {
    let mut cur_set: Option<HashSet<char>> = None;
    for ch in name.chars() {
        if let Some(set) = cur_set
            && !set.contains(&ch)
        {
            return false;
        }
        cur_set = mapping.get(&ch).cloned();
    }
    true
}

#[derive(Default)]
struct Cache {
    cache: HashSet<String>,
}

impl Cache {
    fn find(&mut self, prefix: String, mapping: &HashMap<char, HashSet<char>>) -> usize {
        let last_len = prefix.len();
        if last_len > 11 {
            return 0;
        }
        if last_len >= 7 {
            if !self.cache.insert(prefix.clone()) {
                return 0;
            }
        }

        let last_char = prefix.chars().rev().next().unwrap();
        if let Some(next_charset) = mapping.get(&last_char) {
            for next_char in next_charset {
                let next_prefix = format!("{prefix}{next_char}");
                self.find(next_prefix, mapping);
            }
        }

        self.cache.len()
    }
}

struct Rules {
    names: Vec<String>,
    mapping: HashMap<char, HashSet<char>>,
}

impl Rules {
    fn read<P: AsRef<Path>>(pathname: P) -> Self {
        let lines = aoclib::read_lines(pathname);
        let names = lines[0]
            .split(',')
            .map(|name| name.to_string())
            .collect::<Vec<_>>();
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
