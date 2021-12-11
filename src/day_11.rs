use std::collections::HashSet;

fn calc_flashes(mut e_levels: Vec<Vec<usize>>) -> usize {
    (0..100)
        .into_iter()
        .fold(0, |total, _| total + flash_step(&mut e_levels))
}

fn find_first_sync(mut e_levels: Vec<Vec<usize>>) -> usize {
    let mut i = 0;
    loop {
        i += 1;
        let flashes = flash_step(&mut e_levels);
        if flashes == 10 * 10 {
            return i;
        }
    }
}

fn flash_step(e_levels: &mut Vec<Vec<usize>>) -> usize {
    let mut flashed = HashSet::new();
    for i in 0..10 {
        for j in 0..10 {
            increase_level(e_levels, i, j, &mut flashed);
        }
    }
    let len = flashed.len();
    flashed.into_iter().for_each(|(i, j)| e_levels[i][j] = 0);
    len
}

fn increase_level(
    e_levels: &mut Vec<Vec<usize>>,
    i: usize,
    j: usize,
    flashed: &mut HashSet<(usize, usize)>,
) {
    if flashed.contains(&(i, j)) {
        return;
    }

    e_levels[i][j] += 1;
    if e_levels[i][j] > 9 {
        flashed.insert((i, j));
        neighbors(i, j)
            .into_iter()
            .for_each(|(i, j)| increase_level(e_levels, i, j, flashed));
    }
}

fn neighbors(i: usize, j: usize) -> Vec<(usize, usize)> {
    let i = i as isize;
    let j = j as isize;

    vec![
        (i - 1, j),
        (i + 1, j),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j + 1),
        (i - 1, j - 1),
        (i - 1, j + 1),
        (i + 1, j - 1),
    ]
    .into_iter()
    .filter(|(i, j)| i >= &0 && j >= &0 && i < &10 && j < &10)
    .map(|(i, j)| (i as usize, j as usize))
    .collect()
}

#[cfg(test)]
mod tests {
    

    
    use crate::data_parser::parse_matrix;
    use crate::day_11::{calc_flashes, find_first_sync};

    #[test]
    fn should_calc_flashes_example_data() {
        let e_levels = parse_matrix("input/day_11_example_data");
        assert_eq!(1656, calc_flashes(e_levels));
    }

    #[test]
    fn should_calc_flashes_test_data() {
        let e_levels = parse_matrix("input/day_11_test_data");
        assert_eq!(1691, calc_flashes(e_levels));
    }

    #[test]
    fn should_calc_first_sync_flash_test_data() {
        let e_levels = parse_matrix("input/day_11_test_data");
        assert_eq!(216, find_first_sync(e_levels));
    }
}
