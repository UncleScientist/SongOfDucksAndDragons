fn main() {
    /*
    println!("{}", calc_gear(&[128.0, 64.0, 32.0, 16.0, 8.0]));
    println!(
        "{}",
        (2025.0 * calc_gear(&[102.0, 75.0, 50.0, 35.0, 13.0])).floor()
    );
    */
    let lines: Vec<f64> = aoclib::read_numbers("input/everybody_codes_e2025_q04_p1.txt");
    println!("part 1 = {}", (calc_gear(&lines) * 2025.0).floor());
}

fn calc_gear(ratios: &[f64]) -> f64 {
    let mut result: f64 = 1.0;

    for pair in ratios.windows(2) {
        result *= pair[0] / pair[1];
    }

    result
}
