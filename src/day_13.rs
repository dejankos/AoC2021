type Instruction = (char, usize);

fn calc_first_fold(points: Vec<(usize, usize)>, inst: Vec<Instruction>) -> usize {
    let max_x = points.iter().map(|(x, _)| x).max().unwrap();
    let max_y = points.iter().map(|(_, y)| y).max().unwrap();

    let mut paper = vec![vec!['.'; *max_x + 1]; *max_y + 1];
    for (x, y) in points {
        paper[y][x] = '#';
    }

    for (pos, fold_at) in inst.iter() {
        match pos {
            'y' => fold_y(&mut paper, *fold_at),
            'x' => fold_x(&mut paper, *fold_at),
            _ => unreachable!(""),
        }
    }

    // part 2
    if inst.len() > 1 {
        for row in &paper {
            let string = row.iter().collect::<String>();
            println!("{:?}", string);
        }
    }

    paper
        .into_iter()
        .map(|row| row.into_iter().filter(|c| c == &'#').count())
        .sum()
}

fn fold_y(paper: &mut Vec<Vec<char>>, fold_at_y: usize) {
    let mut removed = vec![];

    let for_removal = paper.len() - fold_at_y;
    for _ in 0..for_removal {
        let r = paper.remove(paper.len() - 1);
        removed.push(r);
    }
    for (i, row) in removed.iter().enumerate() {
        for (j, v) in row.iter().enumerate() {
            if v == &'#' {
                paper[i][j] = *v;
            }
        }
    }
}

fn fold_x(paper: &mut Vec<Vec<char>>, fold_at_x: usize) {
    for (_, row) in paper.iter_mut().enumerate() {
        for i in 0..fold_at_x + 1 {
            if i < fold_at_x {
                let c = row.remove(row.len() - 1);
                if c == '#' {
                    row[i] = c;
                }
            } else {
                row.remove(row.len() - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use std::path::Path;

    use crate::data_parser;
    use crate::day_13::calc_first_fold;

    #[test]
    fn should_calc_calc_first_fold_example_data() {
        let points = parse_data("input/day_13_example_data");
        assert_eq!(17, calc_first_fold(points, vec![('y', 7)]));
    }

    #[test]
    fn should_calc_calc_first_fold_example_data_2() {
        let points = parse_data("input/day_13_example_data");
        calc_first_fold(points, vec![('y', 7), ('x', 5)]);
    }

    #[test]
    fn should_calc_calc_first_fold_test_data() {
        let points = parse_data("input/day_13_test_data");
        assert_eq!(751, calc_first_fold(points, vec![('x', 655)]));
    }

    #[test]
    fn should_calc_calc_first_fold_test_data_2() {
        let points = parse_data("input/day_13_test_data");
        calc_first_fold(
            points,
            vec![
                ('x', 655),
                ('y', 447),
                ('x', 327),
                ('y', 223),
                ('x', 163),
                ('y', 111),
                ('x', 81),
                ('y', 55),
                ('x', 40),
                ('y', 27),
                ('y', 13),
                ('y', 6),
            ],
        );
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<(usize, usize)> {
        data_parser::parse_lines(p)
            .into_iter()
            .map(|line| {
                let pair = line.split(',').collect_vec();
                (
                    pair[0].parse::<usize>().unwrap(),
                    pair[1].parse::<usize>().unwrap(),
                )
            })
            .collect_vec()
    }
}
