use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Point {
    x: usize,
    y: usize,
}

fn calc_overlaps(points: Vec<Vec<Point>>, include_vert: bool) -> usize {
    let mut point_overlaps = HashMap::new();
    points
        .into_iter()
        .filter(|p| include_vert || (p[0].x == p[1].x || p[0].y == p[1].y))
        .flat_map(calc_points)
        .for_each(|p| {
            match point_overlaps.get_mut(&p) {
                Some(v) => {
                    *v += 1;
                }
                _ => {
                    point_overlaps.insert(p, 1);
                }
            };
        });

    point_overlaps.values().filter(|v| *v >= &2).count()
}

fn calc_points(p: Vec<Point>) -> HashSet<Point> {
    let (mut x, mut y) = (p[0].x, p[0].y);

    let mut unique_points = (0..=steps(&p))
        .into_iter()
        .map(|_| {
            x += (x < p[1].x) as usize;
            x -= (x > p[1].x) as usize;

            y -= (y > p[1].y) as usize;
            y += (y < p[1].y) as usize;

            Point { x, y }
        })
        .collect::<HashSet<Point>>();
    unique_points.insert(p[0]);
    unique_points.insert(p[1]);

    unique_points
}

fn steps(p: &[Point]) -> usize {
    let x = p[0].x.max(p[1].x) - p[0].x.min(p[1].x);
    let y = p[0].y.max(p[1].y) - p[0].y.min(p[1].y);
    x.max(y) as usize
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_5::{calc_overlaps, calc_points, Point};

    #[test]
    fn should_calc_overlaps_example_data() {
        let data = parse_data("input/day_5_example_data");
        assert_eq!(5, calc_overlaps(data, false));
    }

    #[test]
    fn should_calc_all_overlaps_example_data() {
        let data = parse_data("input/day_5_example_data");
        assert_eq!(12, calc_overlaps(data, true));
    }

    #[test]
    fn testing() {
        println!(
            "{:?}",
            calc_points(vec![Point { x: 9, y: 7 }, Point { x: 7, y: 9 }])
        )
    }

    #[test]
    fn should_calc_all_overlaps_test_data() {
        let data = parse_data("input/day_5_test_data");
        assert_eq!(21406, calc_overlaps(data, true));
    }

    #[test]
    fn should_calc_overlaps_test_data() {
        let data = parse_data("input/day_5_test_data");
        assert_eq!(7438, calc_overlaps(data, false));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<Vec<Point>> {
        data_parser::parse_lines(p)
            .iter()
            .map(|line| {
                line.split("->")
                    .map(|part| {
                        let part = part
                            .split(',')
                            .map(|n| n.trim().parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();

                        Point {
                            x: part[0],
                            y: part[1],
                        }
                    })
                    .collect::<Vec<Point>>()
            })
            .collect::<Vec<Vec<Point>>>()
    }
}
