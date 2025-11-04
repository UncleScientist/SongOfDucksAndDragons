fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q01_p1.txt");
    let names = lines[0].split(',').collect::<Vec<_>>();
    let inst = lines[1].split(',').collect::<Vec<_>>();

    println!("names = {names:?}");
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
}
