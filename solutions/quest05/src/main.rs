fn main() {
    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q05_p1.txt");
    let (_, nums) = lines[0].split_once(':').unwrap();
    let nums = nums
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("part 1 = {}", calc_quality(&nums));
}

fn calc_quality(nums: &[usize]) -> usize {
    let mut sword: Vec<Element> = Vec::new();
    let mut part1 = String::new();
    for n in nums {
        let mut found = false;
        for s in &mut sword {
            if *n < s.spine && s.left.is_none() {
                s.left = Some(*n);
                found = true;
                break;
            } else if *n > s.spine && s.right.is_none() {
                s.right = Some(*n);
                found = true;
                break;
            }
        }
        if !found {
            sword.push(Element::add(*n));
            part1.push_str(format!("{n}").as_str());
        }
    }

    part1.parse().unwrap()
}

#[derive(Debug, Default)]
struct Element {
    left: Option<usize>,
    spine: usize,
    right: Option<usize>,
}

impl Element {
    fn add(spine: usize) -> Self {
        Self {
            spine,
            ..Self::default()
        }
    }
}
