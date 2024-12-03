advent_of_code::solution!(3);

fn multiply(index: &usize, input: &str) -> Option<(u32, usize)> {
    let start_index = *index + 4;

    if &input[*index..start_index] != "mul(" {
        return None;
    }

    let end_index = input[start_index..].find(")")? + start_index;
    let sep_index = input[start_index..end_index].find(",")? + start_index;

    let a = input[start_index..sep_index].parse::<u32>().ok()?;
    let b = input[sep_index + 1..end_index].parse::<u32>().ok()?;

    Some((a * b, end_index))
}

// ugly but very fast
pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;

    let mut index = 0;
    while index + 4 < input.len() {
        let token = &input[index..index + 4];

        if token == "mul(" {
            match multiply(&index, input) {
                Some((value, end_index)) => {
                    total += value;
                    index = end_index;
                }
                None => {
                    index += 4;
                }
            }
        } else {
            index += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    let mut is_multiplying = true;

    let mut index = 0;
    while index + 4 < input.len() {
        let token = &input[index..index + 4];

        if is_multiplying && token == "mul(" {
            match multiply(&index, input) {
                Some((value, end_index)) => {
                    total += value;
                    index = end_index;
                }
                None => {
                    index += 4;
                }
            }
        }  else if token == "do()" {
            is_multiplying = true;
            index += 4;
        } else if index + 7 < input.len() && &input[index..index + 7] == "don't()" {
            is_multiplying = false;
            index += 7;
        } else {
            index += 1;
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
