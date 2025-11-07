fn main() {
    /*
    println!(
        "{}",
        (0.9999 + 10000000000000.0 / calc_gear(&[128.0, 64.0, 32.0, 16.0, 8.0])).floor()
    );
    println!(
        "{}",
        (0.99999 + 10000000000000.0 / calc_gear(&[102.0, 75.0, 50.0, 35.0, 13.0])).floor()
    );
    */
    let gears: Vec<f64> = aoclib::read_numbers("input/everybody_codes_e2025_q04_p1.txt");
    println!("part 1 = {}", (calc_gear(&gears) * 2025.0).floor());

    let gears: Vec<f64> = aoclib::read_numbers("input/everybody_codes_e2025_q04_p2.txt");
    println!(
        "part 2 = {}",
        (0.99999 + 10000000000000.0 / calc_gear(&gears)).floor()
    );

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q04_p3.txt");
    // let lines = aoclib::read_lines("input/test_3.txt");
    let first = lines[0].parse::<f64>().unwrap();
    let mapping = lines[1..lines.len() - 1]
        .iter()
        .map(|line| {
            let (left, right) = line.split_once('|').unwrap();
            (left.parse::<f64>().unwrap(), right.parse::<f64>().unwrap())
        })
        .collect::<Vec<_>>();
    let last = lines[lines.len() - 1].parse::<f64>().unwrap();
    let mut ratio = first;
    let mut turns = 100.0;
    for (left, right) in mapping {
        ratio /= left;
        turns *= ratio;
        println!("({left}|{right}) {ratio} -> turns = {turns}");
        ratio = right;
    }
    ratio /= last;
    println!("part 3 = {}", (turns * ratio).floor());
}

fn calc_gear(ratios: &[f64]) -> f64 {
    let mut result: f64 = 1.0;

    for pair in ratios.windows(2) {
        result *= pair[0] / pair[1];
    }

    result
}
