use regex::{Regex};

advent_of_code::solution!(3);

const ALL_TOKENS: &str = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let mut index = 0;
    while index < input.len() {
        let token = &input[index..index + 1];

        if token == "m" && &input[index..index + 4] == "mul(" {
            let sep_index = input[index..].find(",");
            let end_index = input[index..].find(")");

            if sep_index.is_none() || end_index.is_none() {
                index += 3;
                continue;
            }

            if end_index? < sep_index? {
                index += 3;
                continue;
            }

            let a = input[index + 4..index + sep_index?].parse::<u32>();
            let b = input[index + sep_index? + 1..index + end_index?].parse::<u32>();

            if a.is_err() || b.is_err() {
                index += 3;
                continue;
            }

            total += a.unwrap() * b.unwrap();
        }

        index += 1;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let token_regex: Regex = Regex::new(ALL_TOKENS).unwrap();

    let mut total: u32 = 0;

    // iterate over all matches
    let mut is_multiplying = true;
    for cap in token_regex.captures_iter(input) {
        let token = &cap[0];

        if token == "do()" {
            is_multiplying = true;
        } else if token == "don't()" {
            is_multiplying = false;
        } else if is_multiplying {
            let a = cap[1].parse::<u32>().expect("Failed to parse integer");
            let b = cap[2].parse::<u32>().expect("Failed to parse integer");
            total += a * b;
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
