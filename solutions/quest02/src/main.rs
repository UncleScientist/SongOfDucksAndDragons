use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
    str::FromStr,
};

const DIVISOR: Complex = Complex(10, 10);

fn main() {
    let lines = aoclib::read_lines("input/everybody_codes_e2025_q02_p1.txt");
    let addend = lines[0][2..].parse().unwrap();
    let mut result = Complex(0, 0);
    for _ in 0..3 {
        result = cycle(&result, &addend);
    }
    println!("part 1 = {result}");
}

fn cycle(c: &Complex, addend: &Complex) -> Complex {
    let mut num = *c * *c;
    num = num / DIVISOR;
    num + *addend
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Complex(i64, i64);

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.0, self.1)
    }
}

impl FromStr for Complex {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').unwrap();
        Ok(Complex(
            left[1..].parse().unwrap(),
            right[..right.len() - 1].parse().unwrap(),
        ))
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex(
            self.0 * rhs.0 - self.1 * rhs.1,
            self.0 * rhs.1 + self.1 * rhs.0,
        )
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, rhs: Self) -> Self::Output {
        Complex(self.0 / rhs.0, self.1 / rhs.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        let a = Complex(-2, 5);
        let b = Complex(10, -1);
        assert_eq!(Complex(8, 4), a + b);
    }

    #[test]
    fn test_multiplication() {
        let a = Complex(-2, 5);
        let b = Complex(10, -1);
        assert_eq!(Complex(-15, 52), a * b);
    }

    #[test]
    fn test_division() {
        let a = Complex(-10, -12);
        let b = Complex(2, 2);
        assert_eq!(Complex(-5, -6), a / b);
    }
}
