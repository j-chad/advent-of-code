advent_of_code::solution!(6);

// (4, 6)

fn index_to_position(index: usize, width: usize) -> (usize, usize) {
    (index % width, index / width)
}

fn position_to_index(position: (usize, usize), width: usize) -> usize {
    position.0 + position.1 * width
}

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited_indexes = std::collections::HashSet::<usize>::new();

    let width = input.lines().next()?.len() + 1;
    let height = input.lines().count();

    let mut current_direction = Direction::Up;
    let mut current_index = input.find('^')?;

    loop {
        if current_index == 0 && (current_direction == Direction::Left || current_direction == Direction::Up) {
            // we've reached the end - index cannot be less than 0
            break;
        }

        let current_position = index_to_position(current_index, width);
        let next_position = match current_direction {
            Direction::Up => (current_position.0, current_position.1 - 1),
            Direction::Down => (current_position.0, current_position.1 + 1),
            Direction::Left => (current_position.0 - 1, current_position.1),
            Direction::Right => (current_position.0 + 1, current_position.1),
        };

        if next_position.0 >= width || next_position.1 >= height {
            // we've reached the end - position is out of bounds
            break;
        }

        visited_indexes.insert(current_index);

        let next_index = position_to_index(next_position, width);
        let next_char = input.chars().nth(next_index).expect("could not get next char");

        if next_char == '#' {
            // hit a wall
            current_direction = match current_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
            continue;
        }
        
        // walk a step
        current_index = next_index;
    }

    Some(visited_indexes.len() as u32)
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
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
