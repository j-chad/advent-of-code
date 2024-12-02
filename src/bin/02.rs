advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");
        let failed_index = check_report(&levels);

        if failed_index.is_none() {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let mut levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");
        let failed_index = check_report(&levels);

        match failed_index {
            Some(index) => {
                levels.remove(index);
                let failed_index = check_report(&levels);
                if failed_index.is_none() {
                    safe += 1;
                }
            }
            None => {
                safe += 1;
                continue;
            }
        }
    }

    Some(safe)
}

// If the report fails - returns the index of the first failed level
fn check_report(levels: &Vec<u32>) -> Option<usize> {
    let mut is_ascending: Option<bool> = None;

    for (index, window) in levels.windows(2).enumerate() {
        let [a, b] = *window else { panic!("invalid window") };

        let ascending = a < b;
        match is_ascending {
            Some(is_ascending) => {
                if is_ascending != ascending {
                    return Some(index); // unsafe - data is fluctuating
                }
            }
            None => {
                is_ascending = Some(ascending);
            }
        }

        let difference = if ascending { b - a } else { a - b };
        if difference < 1 || difference > 3 {
            return Some(index); // unsafe - data changes too much or too little
        }
    }

    None
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
