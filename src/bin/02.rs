advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");
        if report_valid(&levels) {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");

        for index in 0..levels.len() {
            let mut new_levels = levels.clone();
            new_levels.remove(index);

            if report_valid(&new_levels) {
                safe += 1;
                break;
            }
        }
    }

    Some(safe)
}

fn report_valid(levels: &Vec<u32>) -> bool {
    let mut is_ascending: Option<bool> = None;

    for window in levels.windows(2) {
        let [a, b] = *window else { panic!("invalid window") };

        let ascending = a < b;
        match is_ascending {
            Some(is_ascending) => {
                if is_ascending != ascending {
                    return false;
                }
            }
            None => {
                is_ascending = Some(ascending);
            }
        }

        let difference = if ascending { b - a } else { a - b };
        if difference < 1 || difference > 3 {
            return false
        }
    }

    true
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
