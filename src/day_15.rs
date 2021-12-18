use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, matrix: &[Vec<usize>], factor: usize) -> Vec<(Pos, usize)> {
        let (i_len, j_len) = (matrix.len() * factor, matrix[0].len() * factor);
        let &Pos(i, j) = self;
        let i = i as isize;
        let j = j as isize;
        vec![(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
            .into_iter()
            .filter(|(i, j)| i >= &0 && j >= &0)
            .map(|(i, j)| (i as usize, j as usize))
            .filter(|(i, j)| i < &i_len && j < &j_len)
            .map(|(i, j)| {
                let i_dim = i / matrix.len();
                let j_dim = j / matrix[0].len();
                let dim_0_value = matrix[i % matrix.len()][j % matrix[0].len()];
                let mut new_value = dim_0_value + i_dim + j_dim;
                if new_value > 9 {
                    new_value = new_value % 9;
                }
                (Pos(i, j), new_value)
            })
            .collect()
    }
}

fn find_shortest_path(matrix: &[Vec<usize>], factor: usize) -> usize {
    let target = Pos((matrix.len() * factor) - 1, (matrix[0].len() * factor) - 1);
    dijkstra(
        &Pos(0, 0),
        |p| p.successors(matrix, factor),
        |p| *p == target,
    )
    .expect("should find a path")
    .1
}

#[cfg(test)]
mod tests {
    use crate::data_parser::parse_matrix;
    use crate::day_15::find_shortest_path;

    #[test]
    fn should_calc_shortest_path_example_data() {
        let m = parse_matrix("input/day_15_example_data");
        assert_eq!(40, find_shortest_path(&m, 1));
    }

    #[test]
    fn should_calc_shortest_path_test_data() {
        let m = parse_matrix("input/day_15_test_data");
        assert_eq!(423, find_shortest_path(&m, 1));
    }

    #[test]
    fn should_calc_shortest_path_test_data() {
        let m = parse_matrix("input/day_15_test_data");
        assert_eq!(2778, find_shortest_path(&m, 5));
    }
}
