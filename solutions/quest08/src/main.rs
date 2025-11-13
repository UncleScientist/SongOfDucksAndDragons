fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q08_p1.txt");
    let nails = lines[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();
    println!("part 1 = {}", count_centers(&nails, 32));

    let lines = aoclib::read_lines("input/everybody_codes_e2025_q08_p2.txt");
    let nails = lines[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();
    println!("part 2 = {}", count_crossings(&nails, 256));
}

fn count_centers(nails: &[usize], nail_count: usize) -> usize {
    nails
        .windows(2)
        .filter(|pair| pair[0].abs_diff(pair[1]) == nail_count / 2)
        .count()
}

fn count_crossings(nails: &[usize], _nail_count: usize) -> usize {
    let mut strings = Vec::<(usize, usize)>::new();
    let mut crossings = 0;
    for pair in nails.windows(2) {
        assert!(pair[0] != pair[1]);
        let (cur_min, cur_max) = if pair[0] < pair[1] {
            (pair[0], pair[1])
        } else {
            (pair[1], pair[0])
        };
        for &(min, max) in &strings {
            if (cur_min < min && (cur_max < max && cur_max > min))
                || (cur_min > min && cur_min < max && cur_max > max)
            {
                crossings += 1;
            }
        }
        strings.push((cur_min, cur_max));
    }
    crossings
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let list = [1, 5, 2, 6, 8, 4, 1, 7, 3];
        assert_eq!(4, count_centers(&list, 8));
    }

    #[test]
    fn example2() {
        let list = [1, 5, 2, 6, 8, 4, 1, 7, 3, 5, 7, 8, 2];
        assert_eq!(21, count_crossings(&list, 8));
    }
}
