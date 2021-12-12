use std::collections::{HashMap, HashSet, VecDeque};
const START: &str = "start";
const END: &str = "end";

fn find_paths(paths: Vec<(String, String)>) -> usize {
    let lookup = fill_lookup(paths);

    lookup.get(START).unwrap().iter().fold(0, |acc, node| {
        let mut visited = set!();
        acc + find_all_paths(node, &lookup, &mut visited)
    })
}

fn find_paths_2(paths: Vec<(String, String)>) -> usize {
    let lookup = fill_lookup(paths);
    let mut all_paths = lookup[START].iter().fold(VecDeque::new(), |mut vd, e| {
        vd.push_back(vec![e]);
        vd
    });
    let mut total_paths = 0;
    while !all_paths.is_empty() {
        let current = all_paths.pop_front().unwrap();
        if *current.last().unwrap() == END {
            total_paths += 1;
            continue;
        }

        for p in &lookup[current[current.len() - 1]] {
            let mut new_path = current.clone();
            new_path.push(p);
            let mut path_occurences = HashMap::new();
            for i in new_path.iter() {
                if &*i.to_lowercase() == *i {
                    *path_occurences.entry(i).or_insert(0) += 1;
                }
            }

            if path_occurences.values().filter(|p| *p == &2).count() < 2
                && path_occurences.values().filter(|p| *p > &2).count() == 0
            {
                all_paths.push_back(new_path);
            }
        }
    }

    total_paths
}

fn fill_lookup(paths: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut lookup = HashMap::new();
    paths.into_iter().for_each(|(from, to)| {
        if to != START {
            lookup
                .entry(from.clone())
                .or_insert_with(Vec::new)
                .push(to.clone());
        }
        if from != START {
            lookup.entry(to).or_insert_with(Vec::new).push(from);
        }
    });

    lookup
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
        let mut visited = visited.clone();
        acc + find_all_paths(node, lookup, &mut visited)
    })
}

#[cfg(test)]
mod tests {

    use std::path::Path;

    use itertools::Itertools;

    use crate::data_parser;
    use crate::day_12::{find_paths, find_paths_2};

    #[test]
    fn should_calc_paths_example_data() {
        let paths = parse_data("input/day_12_example_data");
        assert_eq!(19, find_paths(paths));
    }

    #[test]
    fn should_calc_paths_2_example_data() {
        let paths = parse_data("input/day_12_example_data");
        assert_eq!(36, find_paths_2(paths));
    }

    #[test]
    fn should_calc_paths_2_test_data() {
        let paths = parse_data("input/day_12_test_data");
        assert_eq!(107395, find_paths_2(paths));
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
