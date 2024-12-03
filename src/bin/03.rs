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
    let mul_regex: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_regex: Regex = Regex::new(r"do\(\)").unwrap();
    let dont_regex: Regex = Regex::new(r"don't\(\)").unwrap();

    let mut total: u32 = 0;

    let dont_matches: Vec<_> = dont_regex.find_iter(input).collect();
    let do_matches: Vec<_> = do_regex.find_iter(input).collect();
    let mul_matches: Vec<_> = mul_regex.find_iter(input).collect();

    // collect all into a single vector
    let mut all_matches: Vec<_> = dont_matches.iter().chain(do_matches.iter()).chain(mul_matches.iter()).collect();

    // sort by start position
    all_matches.sort_by_key(|m| m.start());

    // iterate over all matches
    let mut is_multiplying = true;
    for m in all_matches {
        if mul_matches.contains(&m) && is_multiplying {
            let cap = mul_regex.captures(m.as_str()).unwrap();
            let a = cap[1].parse::<u32>().unwrap();
            let b = cap[2].parse::<u32>().unwrap();
            total += a * b;
        } else if dont_regex.is_match(m.as_str()) {
            is_multiplying = false;
        } else if do_regex.is_match(m.as_str()) {
            is_multiplying = true;
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
