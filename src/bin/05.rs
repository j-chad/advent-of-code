use std::collections::HashMap;

advent_of_code::solution!(5);

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn build_dag<'a>(rules: &[(&'a str, &'a str)], update: &[&str]) -> Graph<'a> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::with_capacity(rules.len());

    for (left, right) in rules {
        if (!update.contains(left)) || (!update.contains(right)) {
            continue;
        }
        
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
    
    if length % 2 != 1 {
        panic!("update length is not odd");
    }
    
    update[middle].parse::<u32>().expect("could not parse middle value")
}

fn build_rules(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|line| {
        line.split_once("|").expect("could not split rule")
    }).collect()
}

fn kahns_topo_sort<'a>(graph: &Graph<'a>) -> Vec<&'a str> {
    let mut in_degree: HashMap<&str, usize> = HashMap::new();
    let mut queue: Vec<&str> = Vec::new();
    let mut sorted: Vec<&str> = Vec::new();

    for (node, _) in graph.iter() {
        in_degree.entry(node).or_insert(0);
    }

    for (_, edges) in graph.iter() {
        for edge in edges {
            *in_degree.entry(edge).or_insert(0) += 1;
        }
    }

    for (node, degree) in in_degree.iter() {
        if *degree == 0 {
            queue.push(node);
        }
    }

    while !queue.is_empty() {
        let node = queue.pop().unwrap();
        sorted.push(node);

        for edge in graph.get(node).unwrap_or(&Vec::new()) {
            let degree = in_degree.get_mut(edge).unwrap();
            *degree -= 1;

            if *degree == 0 {
                queue.push(edge);
            }
        }
    }

    sorted
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    
    let (rules, updates) = input.split_once("\n\n").expect("could not split parts");
    let rules = build_rules(rules);
    
    for update in updates.lines() {
        let update = update.split(",").collect::<Vec<&str>>();
        let graph = build_dag(&rules, &update);
        
        if check_order(&graph, &update) {
            total += get_middle(&update);
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;

    let (rules, updates) = input.split_once("\n\n").expect("could not split parts");
    let rules = build_rules(rules);

    for update in updates.lines() {
        let update = update.split(",").collect::<Vec<&str>>();
        let graph = build_dag(&rules, &update);
        let sorted_nodes = kahns_topo_sort(&graph);

        if !check_order(&graph, &update) {
            let new_order = sorted_nodes.iter().filter(|&node| update.contains(node)).map(|&node| node).collect::<Vec<&str>>();
            total += get_middle(&new_order);
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
