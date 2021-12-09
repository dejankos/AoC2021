fn find_low_points_height_sum(m: Vec<Vec<usize>>) -> usize {
    let mut low_points = vec![];
    for (i, row) in m.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let up = if i > 0 { Some(m[i - 1][j]) } else { None };
            let down = if i < m.len() - 1 {
                Some(m[i + 1][j])
            } else {
                None
            };
            let left = if j > 0 { Some(m[i][j - 1]) } else { None };
            let right = if j < m[i].len() - 1 {
                Some(m[i][j + 1])
            } else {
                None
            };

            let min = vec![up, down, left, right]
                .into_iter()
                .flatten()
                .min()
                .unwrap();

            if val < &min {
                low_points.push(*val);
            }
        }
    }

    low_points.iter().sum::<usize>() + low_points.len()
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use crate::data_parser;
    use crate::day_9::find_low_points_height_sum;

    #[test]
    fn should_find_low_points_sum_example_data() {
        let data = parse_data("input/day_9_example_data");
        assert_eq!(15, find_low_points_height_sum(data));
    }

    #[test]
    fn should_find_low_points_sum() {
        let data = parse_data("/input/day_9_test_input");
        assert_eq!(506, find_low_points_height_sum(data));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<Vec<usize>> {
        data_parser::parse_lines(p)
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse::<usize>().unwrap())
                    .collect()
            })
            .collect()
    }
}
