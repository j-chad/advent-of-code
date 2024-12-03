use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mul_regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut total: u32 = 0;

    // search input for all instances of "mul(a, b)"
    for cap in mul_regex.captures_iter(input) {
        let a = cap[1].parse::<u32>().unwrap();
        let b = cap[2].parse::<u32>().unwrap();
        total += a * b;
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
