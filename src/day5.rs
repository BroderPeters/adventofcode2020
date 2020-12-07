use substring::Substring;

#[aoc_generator(day5)]
pub fn input_generator(_input: &str) -> Vec<String> {
    return _input.lines().map(|x| String::from(x)).collect();
}

pub fn get_new_range(range: &mut [u32; 2], c: char) {
    let range_diff = (range[1] - range[0]) / 2 + 1;
    match c {
        'F' | 'L' => range[1] -= range_diff,
        'B' | 'R' => range[0] += range_diff,
        _ => panic!("Unrecognized char: {}", c),
    }
}

pub fn get_place(mut range: &mut [u32; 2], chars: std::str::Chars, decision_char: char) -> u32 {
    for c in chars {
        get_new_range(&mut range, c);
    }
    match decision_char {
        'F' | 'L' => return range[0],
        'B' | 'R' => return range[1],
        _ => panic!("Unrecognized char: {}", decision_char),
    }
}

pub fn get_seat_ids(_input: &[String]) -> Vec<u32> {
    let mut seat_ids: Vec<u32> = Vec::new();
    for elem in _input {
        let mut row_range: [u32; 2] = [0, 127];
        let row = get_place(
            &mut row_range,
            elem.to_string().substring(0, 6).chars(),
            elem.chars().nth(6).unwrap(),
        );

        let mut column_range: [u32; 2] = [0, 7];
        let column = get_place(
            &mut column_range,
            elem.to_string().substring(7, 9).chars(),
            elem.chars().nth(9).unwrap(),
        );
        seat_ids.push(row * 8 + column);
    }
    return seat_ids;
}

#[aoc(day5, part1)]
pub fn solve_part1(_input: &[String]) -> u32 {
    return *get_seat_ids(_input).iter().max().unwrap();
}

#[aoc(day5, part2)]
pub fn solve_part2(_input: &[String]) -> u32 {
    let seat_ids = get_seat_ids(_input);
    for elem in seat_ids.iter() {
        if !seat_ids.contains(&(elem + 1)) {
            return elem + 1;
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day5.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 858);
        assert_eq!(solve_part2(&input), 557);
    }
}
