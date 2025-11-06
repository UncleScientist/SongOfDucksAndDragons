fn main() {
    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q03_p1.txt");
    let mut nums = lines[0]
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let mut cur = nums[0] - 1;
    let mut total = 0;
    for n in nums {
        if cur != n {
            total += n;
            cur = n;
        }
    }
    println!("part 1 = {total}");
}
