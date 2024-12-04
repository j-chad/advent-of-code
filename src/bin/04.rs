advent_of_code::solution!(4);

// coord = (x, y) = (column, row) = (width, height)
// (0, 0) = top left

fn get_index(size: (usize, usize), coord: (usize, usize)) -> usize {
    let x = coord.0;
    let y = coord.1 * size.0;

    x + y
}

fn get_coord(width: usize, index: usize) -> (usize, usize) {
    let x = index % width;
    let y = index / width;

    (x, y)
}

fn get_lines(size: (usize, usize), coord: (usize, usize)) -> Vec<[usize; 3]> {
    let mut lines = Vec::with_capacity(8);

    let has_space_right = (coord.0 + 3) < size.0;
    let has_space_left = coord.0 >= 3;
    let has_space_down = (coord.1 + 3) < size.1;
    let has_space_up = coord.1 >= 3;

    // horizontal right
    if has_space_right {
        lines.push([(coord.0 + 1, coord.1), (coord.0 + 2, coord.1), (coord.0 + 3, coord.1)]);
    }

    // horizontal left
    if has_space_left {
        lines.push([(coord.0 - 1, coord.1), (coord.0 - 2, coord.1), (coord.0 - 3, coord.1)]);
    }

    // vertical down
    if has_space_down {
        lines.push([(coord.0, coord.1 + 1), (coord.0, coord.1 + 2), (coord.0, coord.1 + 3)]);
    }

    // vertical up
    if has_space_up {
        lines.push([(coord.0, coord.1 - 1), (coord.0, coord.1 - 2), (coord.0, coord.1 - 3)]);
    }

    // diagonal down right
    if has_space_right && has_space_down {
        lines.push([(coord.0 + 1, coord.1 + 1), (coord.0 + 2, coord.1 + 2), (coord.0 + 3, coord.1 + 3)]);
    }

    // diagonal down left
    if has_space_left && has_space_down {
        lines.push([(coord.0 - 1, coord.1 + 1), (coord.0 - 2, coord.1 + 2), (coord.0 - 3, coord.1 + 3)]);
    }

    // diagonal up right
    if has_space_right && has_space_up {
        lines.push([(coord.0 + 1, coord.1 - 1), (coord.0 + 2, coord.1 - 2), (coord.0 + 3, coord.1 - 3)]);
    }

    // diagonal up left
    if has_space_left && has_space_up {
        lines.push([(coord.0 - 1, coord.1 - 1), (coord.0 - 2, coord.1 - 2), (coord.0 - 3, coord.1 - 3)]);
    }

    lines.iter().map(|line| {
        [get_index(size, line[0]), get_index(size, line[1]), get_index(size, line[2])]
    }).collect()
}

fn check_line(input: &str, line: [usize; 3]) -> bool {
    let a = input.chars().nth(line[0]).unwrap();
    let b = input.chars().nth(line[1]).unwrap();
    let c = input.chars().nth(line[2]).unwrap();

    a == 'M' && b == 'A' && c == 'S'
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut occourances = 0;

    let height = input.lines().count();
    let width = input.lines().next()?.chars().count();

    for (index, char) in input.chars().enumerate() {
        if char == 'X' {
            // look for XMAS along all axis
            let coord = get_coord(width, index);
            let lines = get_lines((width, height), coord);

            for line in lines {
                if check_line(input, line) {
                    occourances += 1;
                }
            }
        }
    }

    Some(occourances)
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
