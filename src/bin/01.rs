use std::iter::Map;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut col1 = Vec::<u32>::new();
    let mut col2 = Vec::<u32>::new();

    for line in input.lines() {
        // split the line into two columns
        let mut split = line.split_whitespace();
        let char1 = split.next().expect("no first column");
        let char2 = split.next().expect("no second column");

        // parse the columns into u32
        let num1 = char1.parse::<u32>().expect("could not parse first column");
        let num2 = char2.parse::<u32>().expect("could not parse second column");

        col1.push(num1);
        col2.push(num2);
    }

    // sort
    col1.sort_unstable();
    col2.sort_unstable();

    // get distance
    let mut distance = 0;
    for (a, b) in col1.iter().zip(col2.iter()) {
        if a > b {
            distance += a - b;
        } else {
            distance += b - a;
        }
    }


    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut ids = Vec::<u32>::new();
    let mut location_count = std::collections::HashMap::<u32, u32>::new();

    for line in input.lines() {
        // split the line into two columns
        let mut split = line.split_whitespace();
        let char1 = split.next().expect("no first column");
        let char2 = split.next().expect("no second column");

        // parse the columns into u32
        let num1 = char1.parse::<u32>().expect("could not parse first column");
        let num2 = char2.parse::<u32>().expect("could not parse second column");

        let current_count = location_count.get(&num2).unwrap_or(&0);
        location_count.insert(num2, current_count + 1);

        ids.push(num1);
    }

    let mut similarity: u32 = 0;
    for id in ids {
        let count = location_count.get(&id).unwrap_or(&0);
        similarity += id * count;
    }

    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
