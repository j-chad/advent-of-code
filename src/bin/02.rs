advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let mut is_safe: bool = true;
        let mut is_ascending: Option<bool> = None;

        let levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");
        for window in levels.windows(2) {
            let [a, b] = window else { panic!("invalid window") };

            let ascending = a < b;
            match is_ascending {
                Some(is_ascending) => {
                    if is_ascending != ascending {
                        is_safe = false; // unsafe - data is fluctuating
                        break;
                    }
                }
                None => {
                    is_ascending = Some(ascending);
                }
            }

            let difference = if ascending { b - a } else { a - b };
            if difference < 1 || difference > 3 {
                is_safe = false;
            }
        }

        if is_safe {
            safe += 1;
        }
    }

    Some(safe)
}

pub fn part_two(_input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }
}
