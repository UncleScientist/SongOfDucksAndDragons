fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q01_p1.txt");
    let names = lines[0].split(',').collect::<Vec<_>>();
    let inst = lines[1].split(',').collect::<Vec<_>>();

    let mut index = 0i64;
    for i in inst {
        let dist = i[1..].parse::<i64>().unwrap();
        if i.starts_with('R') {
            index = (index + dist).min(names.len() as i64 - 1);
        } else if i.starts_with('L') {
            index = (index - dist).max(0);
        }
    }
    println!("part 1 = {}", names[index as usize]);

    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q01_p2.txt");
    let names = lines[0].split(',').collect::<Vec<_>>();
    let inst = lines[1].split(',').collect::<Vec<_>>();

    let mut index = 0i64;
    for i in inst {
        let dist = i[1..].parse::<i64>().unwrap();
        if i.starts_with('R') {
            index += dist;
        } else if i.starts_with('L') {
            index -= dist;
        }
    }
    println!(
        "part 2 = {}",
        names[index.rem_euclid(names.len() as i64) as usize]
    );

    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q01_p3.txt");
    let mut names = lines[0].split(',').collect::<Vec<_>>();
    let len = names.len() as i64;
    let inst = lines[1].split(',').collect::<Vec<_>>();
    for i in inst {
        let dist = i[1..].parse::<i64>().unwrap();
        if i.starts_with('R') {
            names.swap(0, (dist % len) as usize);
        } else {
            names.swap(0, (-dist).rem_euclid(len) as usize);
        }
    }
    println!("part 3 = {}", names[0]);
}
