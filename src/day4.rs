extern crate itertools;
extern crate regex;

use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;

#[aoc_generator(day4)]
pub fn input_generator(_input: &str) -> Vec<HashMap<String, String>> {
    let mut passports: Vec<HashMap<String, String>> = Vec::new();
    let mut passport: HashMap<String, String> = HashMap::new();
    let mut passport_field: HashMap<String, String>;
    for elem in _input.lines() {
        if elem.is_empty() {
            if passport.keys().len() > 0 {
                passports.push(passport);
                passport = HashMap::new();
            }
            continue;
        }
        passport_field = elem
            .split_whitespace()
            .flat_map(|l| l.split(':').map(|x| String::from(x)))
            .tuples()
            .collect();
        passport.extend(passport_field);
    }
    if passport.keys().len() > 0 {
        passports.push(passport);
    }
    return passports;
}

#[aoc(day4, part1)]
pub fn solve_part1(_input: &[HashMap<String, String>]) -> u32 {
    return _input
        .iter()
        .filter(|x| {
            x.contains_key("byr")
                && x.contains_key("iyr")
                && x.contains_key("eyr")
                && x.contains_key("hgt")
                && x.contains_key("hcl")
                && x.contains_key("ecl")
                && x.contains_key("pid")
        })
        .count() as u32;
}

#[aoc(day4, part2)]
pub fn solve_part2(_input: &[HashMap<String, String>]) -> u32 {
    // return _input
    //     .iter()
    //     .filter(|x| {
    //         let byr = x.get("byr").unwrap().parse::<i32>().unwrap();
    //         1920 >= byr && byr <= 2002
    //     })
    //     .count() as u32;
    let mut valid = 0;
    for elem in _input {
        if elem.contains_key("byr")
            && elem.contains_key("iyr")
            && elem.contains_key("eyr")
            && elem.contains_key("hgt")
            && elem.contains_key("hcl")
            && elem.contains_key("ecl")
            && elem.contains_key("pid")
        {
            let byr = elem.get("byr").unwrap().parse::<i32>().unwrap();
            let iyr = elem.get("iyr").unwrap().parse::<i32>().unwrap();
            let eyr = elem.get("eyr").unwrap().parse::<i32>().unwrap();
            let hgt = elem.get("hgt").unwrap();
            let hcl = elem.get("hcl").unwrap();
            let ecl = elem.get("ecl").unwrap();
            let pid = elem.get("pid").unwrap();

            if !(1920 <= byr && byr <= 2002)
                || !(2010 <= iyr && iyr <= 2020)
                || !(2020 <= eyr && eyr <= 2030)
            {
                continue;
            }
            if hgt.to_lowercase().ends_with("cm") {
                let cm = hgt.trim_end_matches("cm").parse::<i32>().unwrap();
                if !(150 <= cm && cm <= 193) {
                    continue;
                }
            } else if hgt.to_lowercase().ends_with("in") {
                let inch = hgt.trim_end_matches("in").parse::<i32>().unwrap();
                if !(59 <= inch && inch <= 76) {
                    continue;
                }
            } else {
                continue;
            }
            let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
            if !hcl_re.is_match(hcl) {
                continue;
            }

            match ecl.as_str() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                _ => continue,
            }

            let pid_re = Regex::new(r"^[0-9]{9}$").unwrap();
            if !pid_re.is_match(pid) {
                continue;
            }
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
        let input = include_str!("../input/2020/day4.txt");
        let input = input_generator(input);

        assert_eq!(solve_part1(&input), 182);
        assert_eq!(solve_part2(&input), 109);
    }
}
