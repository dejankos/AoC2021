fn calc_lanterns(states: &[u8], days: usize) -> usize {
    let mut fish_count = [0usize; 9];
    for idx in 0..=8 {
        fish_count[idx] = states.iter().filter(|n| **n as usize == idx).count();
    }
    (0..days).into_iter().for_each(|_day| {
        let mut new = 0;
        for idx in 0..=8 {
            match idx {
                0 => new = fish_count[0],
                _ => fish_count[idx - 1] = fish_count[idx],
            }
        }
        fish_count[6] += new;
        fish_count[8] = new;
    });

    fish_count.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_6::calc_lanterns;

    #[test]
    fn should_calc_example_data() {
        let data = parse_data("input/day_6_example_data");
        assert_eq!(5934, calc_lanterns(&data, 80))
    }

    #[test]
    fn should_calc_test_data() {
        let data = parse_data("input/day_6_test_data");
        assert_eq!(354564, calc_lanterns(&data, 80));
        assert_eq!(1609058859115, calc_lanterns(&data, 256));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<u8> {
        data_parser::load_file(p)
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect()
    }
}
