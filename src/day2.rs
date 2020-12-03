use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{a}-{b} {x}: {pass}")]
pub struct PasswordPhilosophy {
    a: usize,
    b: usize,
    x: char,
    pass: String,
}

#[aoc_generator(day2)]
pub fn input_generator(_input: &str) -> Vec<PasswordPhilosophy> {
    return _input.lines().map(|l| l.parse().unwrap()).collect();
}

#[aoc(day2, part1)]
pub fn solve_part1(_input: &[PasswordPhilosophy]) -> u32 {
    let mut valid: u32 = 0;
    for elem in _input {
        let x: Vec<&str> = elem.pass.matches(elem.x).collect();
        if x.len() >= elem.a && x.len() <= elem.b {
            valid += 1;
        }
    }
    return valid;
}

#[aoc(day2, part2)]
pub fn solve_part2(_input: &[PasswordPhilosophy]) -> u32 {
    let mut valid: u32 = 0;
    for elem in _input {
        if (elem.pass.chars().nth(elem.a - 1).unwrap() == elem.x)
            ^ (elem.pass.chars().nth(elem.b - 1).unwrap() == elem.x)
        {
            valid += 1;
        }
    }
    return valid;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day2.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 640);
        assert_eq!(solve_part2(&input), 472);
    }
}
