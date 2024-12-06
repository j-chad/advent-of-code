use std::collections::HashMap;

advent_of_code::solution!(5);

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn build_dag(rules: &str) -> Graph {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::with_capacity(rules.lines().count());

    for line in rules.lines() {
        let (left, right) = line.split_once("|").expect("could not split rule");

        graph.entry(left)
            .and_modify(|e| { e.push(right); })
            .or_insert(vec![right]);
    }

    graph
}

fn check_order(graph: &Graph, update: &[&str]) -> bool {
    let position_map: HashMap<&str, usize> = update.iter().enumerate().map(|(i, &v)| (v, i)).collect();
    
    for (left, rights) in graph.iter() {
        let left_position = position_map.get(left);
        if left_position.is_none() {
            continue;
        }

        for right in rights {
            if let Some(right_position) = position_map.get(right) {
                if left_position.unwrap() > right_position {
                    return false;
                }
            }
        }
    }
    
    true
}

fn get_middle(update: &[&str]) -> u32 {
    let length = update.len();
    let middle = length / 2;
    
    update[middle].parse::<u32>().expect("could not parse middle value")
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    
    let (rules, updates) = input.split_once("\n\n").expect("could not split parts");

    let graph = build_dag(rules);
    
    for update in updates.lines() {
        let update = update.split(",").collect::<Vec<&str>>();
        
        if check_order(&graph, &update) {
            total += get_middle(&update);
        }
    }

    Some(total)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
