use std::cmp::Ordering;

fn main() {
    // let lines = aoclib::read_lines("input/test_1.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q05_p1.txt");
    let (_, nums) = lines[0].split_once(':').unwrap();
    let nums = nums
        .split(',')
        .map(|num| num.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    println!("part 1 = {}", calc_quality(&nums));

    // let lines = aoclib::read_lines("input/test_2.txt");
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q05_p2.txt");
    let mut min = usize::MAX;
    let mut max = 0;
    for line in lines {
        let (_, nums) = line.split_once(':').unwrap();
        let nums = nums
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let quality = calc_quality(&nums);
        min = min.min(quality);
        max = max.max(quality);
    }
    println!("part 2 = {}", max - min);

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q05_p3.txt");
    // let lines = aoclib::read_lines("input/test_3.txt");
    let mut swords: Vec<Sword> = Vec::new();
    for line in lines {
        let (id, nums) = line.split_once(':').unwrap();
        let nums = nums
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        swords.push(Sword::construct(id.parse().unwrap(), &nums));
    }
    swords.sort();
    for s in &swords {
        println!("{:3} | {}", s.id, s.quality);
    }
    println!(
        "part 3 = {}",
        swords
            .iter()
            .enumerate()
            .map(|(pos, Sword { id, .. })| (pos + 1) * *id)
            .sum::<usize>()
    );
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

    fn score(&self) -> usize {
        match (self.left, self.spine, self.right) {
            (None, n, None) => n,
            (None, n, Some(r)) => format!("{n}{r}").parse().unwrap(),
            (Some(l), n, None) => format!("{l}{n}").parse().unwrap(),
            (Some(l), n, Some(r)) => format!("{l}{n}{r}").parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct Sword {
    id: usize,
    quality: usize,
    shaft: Vec<Element>,
}

impl Sword {
    fn construct(id: usize, nums: &[usize]) -> Self {
        let mut shaft: Vec<Element> = Vec::new();
        let mut quality = String::new();

        for n in nums {
            let mut found = false;
            for s in &mut shaft {
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
                quality.push_str(format!("{n}").as_str());
                shaft.push(Element::add(*n));
            }
        }
        let quality = quality.parse().unwrap();

        Self { id, quality, shaft }
    }
}

impl PartialEq for Sword {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}

impl Eq for Sword {}

impl Ord for Sword {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Sword {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.quality > other.quality {
            return Some(Ordering::Less);
        }
        if self.quality < other.quality {
            return Some(Ordering::Greater);
        }

        if self.shaft.len() > other.shaft.len() {
            return Some(Ordering::Less);
        }
        if self.shaft.len() < other.shaft.len() {
            return Some(Ordering::Greater);
        }

        for idx in 0..self.shaft.len() {
            let this = self.shaft[idx].score();
            let that = other.shaft[idx].score();
            if this > that {
                return Some(Ordering::Less);
            }
            if this < that {
                return Some(Ordering::Greater);
            }
        }

        assert!(self.id != other.id);
        if self.id > other.id {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}
