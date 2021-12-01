use itertools::Itertools;

fn count_increases(depths: &[usize]) -> usize {
    depths
        .iter()
        .tuple_windows::<(&usize, &usize)>()
        .fold(0, |acc, (prev, curr)| acc + if curr > prev { 1 } else { 0 })
}

fn count_sum_increases(depths: &[usize]) -> usize {
    let sums = depths
        .iter()
        .tuple_windows::<(&usize, &usize, &usize)>()
        .map(|t| t.0 + t.1 + t.2)
        .collect::<Vec<usize>>();

    count_increases(&sums)
}

#[cfg(test)]
mod tests {
    use crate::data_parser;

    use super::*;

    #[test]
    fn should_find_increases_example_input() {
        assert_eq!(
            7,
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263])
        );
    }

    #[test]
    fn should_find_increases_day_1_input() {
        assert_eq!(
            1477,
            count_increases(&data_parser::parse_file("input/day_1_data.txt"))
        );
    }

    #[test]
    fn should_find_increases_day_1_part_2_input() {
        assert_eq!(
            1523,
            count_sum_increases(&data_parser::parse_file("input/day_1_part_2_data.txt"))
        );
    }
}
