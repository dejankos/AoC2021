use itertools::Itertools;
use std::collections::HashSet;

fn find_low_points_height_sum(m: Vec<Vec<usize>>) -> usize {
    let low_points = find_low_points_coordinates(&m);

    low_points.len() + low_points.into_iter().map(|(i, j)| m[i][j]).sum::<usize>()
}

fn find_low_points_coordinates(m: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    for (i, row) in m.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            let min = find_nearby(m, i, j)
                .into_iter()
                .map(|(val, _, _)| val)
                .min()
                .unwrap();

            if val < &min {
                low_points.push((i, j));
            }
        }
    }

    low_points
}

fn find_basins(m: Vec<Vec<usize>>) -> usize {
    let low_points = find_low_points_coordinates(&m);

    let mut sizes = vec![];
    for point in low_points {
        let val = m[point.0][point.1];
        let mut points = HashSet::new();
        find(&m, point.0, point.1, val, &mut points);

        sizes.push(points.len());
    }

    sizes.sort_unstable();
    sizes.into_iter().rev().take(3).product()
}

fn find(
    m: &[Vec<usize>],
    i: usize,
    j: usize,
    current: usize,
    visited: &mut HashSet<(usize, usize)>,
) {
    visited.insert((i, j));
    let values = find_nearby(m, i, j)
        .into_iter()
        .filter(|(_, i, j)| !visited.contains(&(*i, *j)))
        .collect::<Vec<(usize, usize, usize)>>();

    for v in values {
        if v.0 != 9 && v.0 > current {
            find(m, v.1, v.2, v.0, visited);
        }
    }
}

fn find_nearby(m: &[Vec<usize>], i: usize, j: usize) -> Vec<(usize, usize, usize)> {
    let up = if i > 0 {
        Some((m[i - 1][j], i - 1, j))
    } else {
        None
    };
    let down = if i < m.len() - 1 {
        Some((m[i + 1][j], i + 1, j))
    } else {
        None
    };
    let left = if j > 0 {
        Some((m[i][j - 1], i, j - 1))
    } else {
        None
    };
    let right = if j < m[i].len() - 1 {
        Some((m[i][j + 1], i, j + 1))
    } else {
        None
    };

    vec![right, down, left, up]
        .into_iter()
        .flatten()
        .collect_vec()
}

#[cfg(test)]
mod tests {
    

    
    use crate::data_parser::parse_matrix;
    use crate::day_9::{find_basins, find_low_points_height_sum};

    #[test]
    fn should_find_low_points_sum_example_data() {
        let data = parse_matrix("input/day_9_example_data");
        assert_eq!(15, find_low_points_height_sum(data));
    }

    #[test]
    fn should_find_basins() {
        let data = parse_matrix("input/day_9_example_data");
        assert_eq!(1134, find_basins(data));
    }

    #[test]
    fn should_find_basins_2() {
        let data = parse_matrix("input/day_9_test_input");
        assert_eq!(931200, find_basins(data));
    }

    #[test]
    fn should_find_low_points_sum() {
        let data = parse_matrix("input/day_9_test_input");
        assert_eq!(506, find_low_points_height_sum(data));
    }
}
