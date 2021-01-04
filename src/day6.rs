use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(_input: &str) -> Vec<Vec<String>> {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for elem in _input.lines() {
        if elem.is_empty() {
            groups.push(group.iter().cloned().collect());
            group.clear();
            continue;
        }
        group.push(elem.to_string());
    }
    groups.push(group);
    return groups;
}

#[aoc(day6, part1)]
pub fn solve_part1(_input: &[Vec<String>]) -> u32 {
    let mut sum_yes: u32 = 0;
    for group in _input {
        let mut any_answer: HashSet<char> = HashSet::new();
        for answer in group {
            for c in answer.chars() {
                any_answer.insert(c);
            }
        }
        sum_yes += any_answer.len() as u32;
    }
    return sum_yes;
}

#[aoc(day6, part2)]
pub fn solve_part2(_input: &[Vec<String>]) -> u32 {
    let mut sum_yes: u32 = 0;
    for group in _input {
        let group_members = group.len() as u32;
        let mut group_answers: HashMap<char, u32> = HashMap::new();
        for answer in group {
            for c in answer.chars() {
                let character_findings = group.iter().filter(|x| x.contains(c)).count() as u32;
                group_answers.insert(c, character_findings);
            }
        }
        for (_, val) in group_answers.iter() {
            if *val == group_members {
                sum_yes += 1;
            }
        }
    }
    return sum_yes;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let input = include_str!("../input/2020/day6.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 6799);
        assert_eq!(solve_part2(&input), 3354);
    }
}
