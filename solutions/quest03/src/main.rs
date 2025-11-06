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

    // let lines = aoclib::read_lines("input/test_2.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q03_p2.txt");
    let mut nums = lines[0]
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let mut cur = nums[0] - 1;
    let mut total = 0;
    let mut count = 0;
    for n in nums {
        if cur != n {
            total += n;
            cur = n;
            count += 1;
            if count == 20 {
                break;
            }
        }
    }
    println!("part 2 = {total}");

    // let lines = aoclib::read_lines("input/test_3.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q03_p3.txt");
    let mut nums = lines[0]
        .split(',')
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    nums.sort();
    let mut sets = 0;
    while !nums.is_empty() {
        let mut next_nums = Vec::new();
        sets += 1;
        let mut cur = nums[0] - 1;
        for n in nums {
            if cur != n {
                cur = n;
            } else {
                next_nums.push(n);
            }
        }
        nums = next_nums;
    }
    println!("part 3 = {sets}");
}
