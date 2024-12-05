advent_of_code::solution!(4);

// coord = (y, x) = (row, column) = (height, width)
// (0, 0) = top left

fn get_index(size: (usize, usize), coord: (usize, usize)) -> usize {
    let x = coord.1;
    let y = coord.0 * size.1;

    x + y
}

fn get_coord(width: usize, index: usize) -> (usize, usize) {
    let x = index % width;
    let y = index / width;

    (y, x)
}

fn get_lines(size: (usize, usize), coord: (usize, usize)) -> Vec<[(usize, usize); 3]> {
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

    lines
}

fn get_letter(matrix: &[Vec<char>], coord: (usize, usize)) -> char {
    if coord.0 >= matrix.len() || coord.1 >= matrix[0].len() {
        return ' ';
    }
    
    matrix[coord.0][coord.1]
}

fn check_cross(matrix: &[Vec<char>], start: (usize, usize)) -> bool {
    let top_left= get_letter(matrix, (start.0 - 1, start.1 - 1));
    let top_right = get_letter(matrix, (start.0 - 1, start.1 + 1));
    let bottom_left = get_letter(matrix, (start.0 + 1, start.1 - 1));
    let bottom_right = get_letter(matrix, (start.0 + 1, start.1 + 1));
    
    if (top_left != 'M' && top_left != 'S') || (top_right != 'M' && top_right != 'S') || (bottom_left != 'M' && bottom_left != 'S') || (bottom_right != 'M' && bottom_right != 'S') {
        return false;
    }
    
    if top_left == 'M' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'S' {
        return true;
    }
    
    if top_left == 'S' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'M' {
        return true;
    }
    
    if top_left == 'M' && top_right == 'M' && bottom_left == 'S' && bottom_right == 'S' {
        return true;
    }
    
    if top_left == 'S' && top_right == 'S' && bottom_left == 'M' && bottom_right == 'M' {
        return true;
    }
    
    false
}

fn check_line(matrix: &Vec<Vec<char>>, line: [(usize, usize); 3]) -> bool {
    let a = matrix[line[0].0][line[0].1];
    let b = matrix[line[1].0][line[1].1];
    let c = matrix[line[2].0][line[2].1];

    a == 'M' && b == 'A' && c == 'S'
}

fn get_matrix(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.trim_end().chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let matrix = get_matrix(input);

    let mut occourances = 0;

    let height = matrix.len();
    let width = matrix[0].len();

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char != &'X' {
                continue;
            }

            let coord = (y, x);
            let lines = get_lines((width, height), coord);

            for line in lines {
                if check_line(&matrix, line) {
                    occourances += 1;
                }
            }
        }
    }

    Some(occourances)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matrix = get_matrix(input);

    let mut occourances = 0;

    for (y, row) in matrix.iter().enumerate() {
        for (x, char) in row.iter().enumerate() {
            if char != &'A' {
                continue;
            }

            let coord = (y, x);
            if check_cross(&matrix, coord) {
                occourances += 1;
            }
        }
    }

    Some(occourances)
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
        assert_eq!(result, Some(9));
    }
}
