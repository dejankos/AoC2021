use std::collections::{HashMap};

fn calc_digits(seg: &[String]) -> usize {
    seg.iter().fold(0, |acc, s| match s.len() {
        2 | 3 | 4 | 7 => acc + 1,
        _ => acc,
    })
}

fn calc_digits_2(seg: Vec<(Vec<String>, Vec<String>)>) -> usize {
    seg.into_iter().map(map_digits).sum()
}

fn map_digits(seg: (Vec<String>, Vec<String>)) -> usize {
    let mut lookup = HashMap::new();

    seg.0
        .iter()
        .filter_map(|s| map_seg_to_known_digit(s))
        .for_each(|val| {
            lookup.insert(val.0, val.1);
        });

    let mut first_seg = seg.0;
    first_seg.sort_by_key(|f| f.len());

    for s in first_seg.iter().rev() {
        let found = match s.len() {
            6 => {
                if contains_all(s, &lookup[&4]) {
                    9
                } else if contains_all(s, &lookup[&1]) {
                    0
                } else {
                    6
                }
            }
            5 => {
                if contains_all(&lookup[&6], s) {
                    5
                } else if contains_all(&lookup[&9], s) {
                    3
                } else {
                    2
                }
            }
            _ => 99 // problems and part 2 is all of them" ,
        };
        if found != 99 {
            lookup.insert(found, s.into());
        }
    }

    let mut r = String::new();
    for s in seg.1.iter() {
        for (k, v) in lookup.iter() {
            if contains_all(v, s) && contains_all(s, v) {
                r.push_str(&k.to_string());
                break;
            }
        }
    }

    r.parse::<usize>().unwrap()
}

fn contains_all(f: &str, s: &str) -> bool {
    s.chars().all(|c| f.contains(c))
}

fn map_seg_to_known_digit(seg: &str) -> Option<(u8, String)> {
    match seg.len() {
        3 => Some((7, seg.to_string())),
        4 => Some((4, seg.to_string())),
        7 => Some((8, seg.to_string())),
        2 => Some((1, seg.to_string())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use itertools::Itertools;

    use crate::data_parser;
    use crate::day_8::{calc_digits, calc_digits_2};

    #[test]
    fn should_calc_digits_example_data() {
        let data = parse_data("input/day_8_example_data");
        assert_eq!(26, calc_digits(&data));
    }

    #[test]
    fn should_calc_digits_example_data_part_2() {
        let data = parse_data_2("input/day_8_example_data");
        assert_eq!(61229, calc_digits_2(data));
    }

    #[test]
    fn should_calc_digits_test_data_part_2() {
        let data = parse_data_2("input/day_8_test_data");
        assert_eq!(1011284, calc_digits_2(data));
    }

    #[test]
    fn should_calc_digits_test_data() {
        let data = parse_data("input/day_8_test_data");
        assert_eq!(532, calc_digits(&data));
    }

    fn parse_data_2<P: AsRef<Path>>(p: P) -> Vec<(Vec<String>, Vec<String>)> {
        data_parser::parse_lines(p)
            .iter()
            .map(|line| {
                let split = line.split('|').collect_vec();
                let map = |part: &str| part.split_whitespace().map(|s| s.to_owned()).collect();

                (map(split[0]), map(split[1]))
            })
            .collect()
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<String> {
        data_parser::parse_lines(p)
            .iter()
            .flat_map(|line| line.split('|').collect_vec()[1].split_whitespace())
            .map(|s| s.into())
            .collect()
    }
}
