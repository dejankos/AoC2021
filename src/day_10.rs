use std::collections::VecDeque;

use itertools::Itertools;

fn calc_error_score(lines: Vec<String>) -> usize {
    lines.iter().map(map_line).map(|(v, _)| v).sum()
}

fn calc_part_2(lines: Vec<String>) -> usize {
    let mut all_points = lines
        .into_iter()
        .filter(|l| map_line(l).1)
        .map(|line| {
            let mut stack = VecDeque::new();
            let mut chars = vec![];
            for c in line.chars().rev() {
                match c {
                    '(' => {
                        if stack.pop_back().is_none() {
                            chars.push(')');
                        }
                    }
                    '[' => {
                        if stack.pop_back().is_none() {
                            chars.push(']');
                        }
                    }
                    '{' => {
                        if stack.pop_back().is_none() {
                            chars.push('}');
                        }
                    }
                    '<' => {
                        if stack.pop_back().is_none() {
                            chars.push('>');
                        }
                    }
                    _ => {
                        stack.push_back(c);
                    }
                };
            }
            chars.into_iter().fold(0, |acc, c| match c {
                ')' => (acc * 5) + 1,
                ']' => (acc * 5) + 2,
                '}' => (acc * 5) + 3,
                '>' => (acc * 5) + 4,
                _ => unreachable!(),
            })
        })
        .collect_vec();
    all_points.sort_unstable();
    all_points[all_points.len() / 2]
}

fn map_line(line: &String) -> (usize, bool) {
    let mut stack = VecDeque::new();
    let mut val = 0;
    for c in line.chars() {
        match c {
            ')' => {
                if let Some(v) = stack.pop_back() {
                    if v != '(' {
                        val = map_point(c);
                        break;
                    }
                }
            }
            ']' => {
                if let Some(v) = stack.pop_back() {
                    if v != '[' {
                        val = map_point(c);
                        break;
                    }
                }
            }
            '}' => {
                if let Some(v) = stack.pop_back() {
                    if v != '{' {
                        val = map_point(c);
                        break;
                    }
                }
            }
            '>' => {
                if let Some(v) = stack.pop_back() {
                    if v != '<' {
                        val = map_point(c);
                        break;
                    }
                }
            }
            _ => {
                stack.push_back(c);
            }
        };
    }
    (val, val == 0)
}

fn map_point(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_10::{calc_error_score, calc_part_2};

    #[test]
    fn should_calc_error_score_example_data() {
        assert_eq!(
            26397,
            calc_error_score(parse_data("input/day_10_example_data"))
        );
    }

    #[test]
    fn should_calc_part_2_example_data() {
        assert_eq!(288957, calc_part_2(parse_data("input/day_10_example_data")));
    }

    #[test]
    fn should_calc_part_2_test_data() {
        assert_eq!(
            3539961434,
            calc_part_2(parse_data("input/day_10_test_data"))
        );
    }

    #[test]
    fn should_calc_error_score_test_data() {
        assert_eq!(
            388713,
            calc_error_score(parse_data("input/day_10_test_data"))
        );
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<String> {
        data_parser::parse_lines(p)
    }
}
