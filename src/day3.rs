pub struct Sleigh {
    x: usize,
    y: usize,
}

#[aoc_generator(day3)]
pub fn input_generator(_input: &str) -> Vec<String> {
    return _input.lines().map(|l| l.parse().unwrap()).collect();
}

pub fn sled(_input: &[String], right: usize, down: usize) -> u32 {
    let level_range = _input[0].len() - 1;
    let mut sleigh = Sleigh { x: 0, y: 0 };
    let mut tree_encounters = 0;
    for elem in _input.iter().step_by(down) {
        if elem.chars().nth(sleigh.x).unwrap() == '#' {
            tree_encounters += 1;
        }
        let level_range_distance = level_range - sleigh.x;
        if level_range_distance < right {
            sleigh.x = right - 1 - level_range_distance;
        } else {
            sleigh.x += right;
        }

        sleigh.y += down;
    }
    return tree_encounters;
}

#[aoc(day3, part1)]
pub fn solve_part1(_input: &[String]) -> u32 {
    return sled(_input, 3, 1);
}

#[aoc(day3, part2)]
pub fn solve_part2(_input: &[String]) -> u32 {
    return sled(_input, 1, 1)
        * sled(_input, 3, 1)
        * sled(_input, 5, 1)
        * sled(_input, 7, 1)
        * sled(_input, 1, 2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day3.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 259);
        assert_eq!(solve_part2(&input), 2224913600);
    }
}
