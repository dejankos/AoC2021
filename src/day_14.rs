use itertools::Itertools;

use std::collections::HashMap;

fn calc_polymers(polymer: &str, rules: HashMap<String, String>) -> usize {
    let mut polymer = polymer.to_string();
    for _i in 0..10 {
        let mut current = String::new();
        let pairs = polymer
            .chars()
            .tuple_windows::<(char, char)>()
            .collect_vec();
        let pairs_len = pairs.len();
        for (i, (f, s)) in pairs.into_iter().enumerate() {
            let lookup_key = &format!("{}{}", f, s);
            let middle = rules.get(lookup_key).unwrap();
            current.push_str(&format!("{}{}", f, middle));
            if pairs_len - 1 == i {
                current.push_str(&format!("{}", s));
            }
        }
        polymer = current;
    }

    let mut counts = HashMap::new();
    polymer.chars().for_each(|c| {
        *counts.entry(c).or_insert(0) += 1;
    });

    let (min, max) = counts.values().into_iter().minmax().into_option().unwrap();
    max - min
}

fn calc_polymers_2(polymer: &str, rules: HashMap<String, String>) -> usize {
    let mut char_counts = HashMap::new();
    let mut pair_counts = HashMap::new();
    let mut pairs_counts_temp = HashMap::new();

    polymer
        .chars()
        .for_each(|c| *char_counts.entry(c).or_insert(0) += 1);
    polymer
        .chars()
        .tuple_windows::<(char, char)>()
        .for_each(|(f, s)| {
            *pair_counts.entry((f, s)).or_insert(0) += 1;
        });

    for _ in 0..40 {
        std::mem::swap(&mut pair_counts, &mut pairs_counts_temp);
        pair_counts.clear();

        pairs_counts_temp.iter().for_each(|((f, s), v)| {
            let middle = rules
                .get(&format!("{}{}", f, s))
                .unwrap()
                .parse::<char>()
                .unwrap();
            *char_counts.entry(middle).or_insert(0) += v;

            let l = (*f, middle);
            let r = (middle, *s);

            *pair_counts.entry(l).or_insert(0) += v;
            *pair_counts.entry(r).or_insert(0) += v;
        });
    }

    let (min, max) = char_counts
        .values()
        .into_iter()
        .minmax()
        .into_option()
        .unwrap();
    max - min
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::path::Path;

    use itertools::Itertools;

    use crate::data_parser;
    use crate::day_14::calc_polymers;

    #[test]
    fn should_calc_polymers_example_data() {
        let map = parse_data("input/day_14_example_data");
        assert_eq!(1588, calc_polymers("NNCB", map));
    }

    #[test]
    fn should_calc_polymers_test_data() {
        let map = parse_data("input/day_14_test_data");
        assert_eq!(4517, calc_polymers("SFBBNKKOHHHPFOFFSPFV", map));
    }

    #[test]
    fn should_calc_polymers_test_data_part_2() {
        let map = parse_data("input/day_14_test_data");
        assert_eq!(4704817645083, calc_polymers("SFBBNKKOHHHPFOFFSPFV", map));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> HashMap<String, String> {
        let mut rules = HashMap::new();

        data_parser::parse_lines(p).into_iter().for_each(|line| {
            let pair = line.split("->").collect_vec();
            rules.insert(pair[0].trim().into(), pair[1].trim().into());
        });

        rules
    }
}
