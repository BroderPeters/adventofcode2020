#[aoc_generator(day1)]
pub fn input_generator(_input: &str) -> Vec<i32> {
    return _input.lines().map(|l| l.parse().unwrap()).collect();
}

#[aoc(day1, part1)]
pub fn solve_part1(_input: &[i32]) -> i32 {
    let mut index = 0;
    while index < _input.len() {
        let mut index2 = index + 1;
        while index2 < _input.len() {
            if _input[index] + _input[index2] == 2020 {
                return _input[index] * _input[index2];
            }
            index2 += 1;
        }
        index += 1;
    }
    return 0;
}

#[aoc(day1, part1, refactor)]
pub fn solve_part1_refactor(_input: &[i32]) -> i32 {
    for elem in _input {
        let other_value = 2020 - elem;
        if _input.contains(&other_value) {
            return elem * other_value;
        }
    }
    return 0;
}

#[aoc(day1, part2)]
pub fn solve_part2(_input: &[i32]) -> i32 {
    let mut index = 0;
    while index < _input.len() {
        let mut index2 = index + 1;
        while index2 < _input.len() {
            let mut index3 = index2 + 1;
            while index3 < _input.len() {
                if _input[index] + _input[index2] + _input[index3] == 2020 {
                    return _input[index] * _input[index2] * _input[index3];
                }
                index3 += 1;
            }
            index2 += 1;
        }
        index += 1;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day1.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 935419);
        assert_eq!(solve_part1_refactor(&input), 935419);
        assert_eq!(solve_part2(&input), 49880012);
    }
}
