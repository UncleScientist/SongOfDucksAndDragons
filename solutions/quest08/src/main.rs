fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q08_p1.txt");
    let nails = lines[0]
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect::<Vec<_>>();
    println!("part 1 = {}", count_centers(&nails, 32));
}

fn count_centers(nails: &[usize], nail_count: usize) -> usize {
    nails
        .windows(2)
        .filter(|pair| pair[0].abs_diff(pair[1]) == nail_count / 2)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let list = [1, 5, 2, 6, 8, 4, 1, 7, 3];
        assert_eq!(4, count_centers(&list, 8));
    }
}
