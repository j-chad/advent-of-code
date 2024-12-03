use regex::{Regex};

advent_of_code::solution!(3);

static MUL_TOKEN: &str = r"mul\((\d+),(\d+)\)";
const ALL_TOKENS: &str = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex: Regex = Regex::new(MUL_TOKEN).unwrap();

    let mut total: u32 = 0;

    // search input for all instances of "mul(a, b)"
    for cap in mul_regex.captures_iter(input) {
        let a = cap[1].parse::<u32>().expect("Failed to parse integer");
        let b = cap[2].parse::<u32>().expect("Failed to parse integer");
        total += a * b;
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
