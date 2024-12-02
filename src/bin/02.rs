advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe: u32 = 0;
    let reports = input.lines();

    for report in reports {
        let levels = report.split_whitespace().map(str::parse::<u32>).collect::<Result<Vec<u32>, _>>().expect("invalid report");
        let failed_index = check_report(&levels);

        if failed_index.len() == 0 {
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
        let failed_index = check_report(&levels);

        if failed_index.len() == 0 {
            safe += 1;
            println!("safe report: {:?}", levels);
            continue;
        } else {
            println!("failed initial report: {:?}", levels);
        }

        for index in failed_index {
            let mut new_levels = levels.clone();
            new_levels.remove(index as usize);

            let failed_index = check_report(&new_levels);
            if failed_index.len() == 0 {
                println!("fixed report: {:?}", new_levels);
                safe += 1;
                break;
            } else {
                println!("failed report: {:?}", new_levels);
            }
        }
    }

    Some(safe)
}

// If the report fails - returns index(s) which failed
fn check_report(levels: &Vec<u32>) -> Vec<u32> {
    let max_index = levels.len() - 1;
    let mut is_ascending: Option<bool> = None;

    for (index, window) in levels.windows(2).enumerate() {
        let [a, b] = *window else { panic!("invalid window") };

        let ascending = a < b;
        match is_ascending {
            Some(is_ascending) => {
                if is_ascending != ascending {
                    if index == max_index {
                        return vec![index as u32];
                    }

                    return vec![index as u32, (index + 1) as u32];
                }
            }
            None => {
                is_ascending = Some(ascending);
            }
        }

        let difference = if ascending { b - a } else { a - b };
        if difference < 1 || difference > 3 {
            if index == max_index {
                return vec![index as u32];
            }

            return vec![index as u32, (index + 1) as u32];
        }
    }

    vec![]
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
        assert_eq!(result, Some(8));
    }
}
