use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

const START: &str = "start";
const END: &str = "end";

fn find_paths(paths: Vec<(String, String)>) -> usize {
    let mut lookup = HashMap::new();
    paths.into_iter().for_each(|(from, to)| {
        if to != START {
            fill_lookup(&mut lookup, from.clone(), to.clone());
        }
        if from != START {
            fill_lookup(&mut lookup, to, from);
        }
    });

    lookup.get(START).unwrap().iter().fold(0, |acc, node| {
        let mut visited = set!();
        acc + find_all_paths(node, &lookup, &mut visited)
    })
}

fn fill_lookup(lookup: &mut HashMap<String, Vec<String>>, from: String, to: String) {
    if !lookup.contains_key(&from) {
        lookup.insert(from.clone(), vec![to]);
    } else {
        lookup.get_mut(&from).unwrap().push(to);
    }
}

fn find_all_paths(
    node: &str,
    lookup: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> usize {
    if node == END {
        return 1;
    }
    if node != START && visited.contains(node) {
        return 0;
    }
    if node != START && node.to_lowercase() == node {
        visited.insert(node.to_string());
    }

    lookup.get(node).unwrap().iter().fold(0, |acc, node| {
        let mut visited = HashSet::from_iter(visited.clone().into_iter());
        acc + find_all_paths(node, lookup, &mut visited)
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use itertools::Itertools;

    use crate::data_parser;
    
    use crate::day_12::find_paths;

    #[test]
    fn should_calc_paths_example_data() {
        let paths = parse_data("input/day_12_example_data");
        assert_eq!(19, find_paths(paths));
    }

    #[test]
    fn should_calc_paths_test_data() {
        let paths = parse_data("input/day_12_test_data");
        assert_eq!(3679, find_paths(paths));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<(String, String)> {
        data_parser::parse_lines(p)
            .into_iter()
            .map(|line| {
                let pair = line.split('-').collect_vec();
                (pair[0].to_owned(), pair[1].to_owned())
            })
            .collect_vec()
    }
}
