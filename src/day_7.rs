fn calc_fuel(pos: &mut [u32]) -> u32 {
    let med = median(pos);
    pos.iter().fold(0, |acc, p| acc + p.max(&med) - p.min(&med))
}

fn median(numbers: &mut [u32]) -> u32 {
    numbers.sort_unstable();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn calc_fuel_part_2(pos: &mut [u32]) -> u32 {
    let mut total_min = u32::MAX;
    for i in 0..*pos.iter().max().unwrap() {
        let pos_min = pos.iter().fold(0, |acc, p| {
            let steps = p.max(&i) - p.min(&i);
            acc + (1..=steps).into_iter().sum::<u32>()
        });
        if pos_min < total_min {
            total_min = pos_min;
        }
    }

    total_min
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_7::{calc_fuel, calc_fuel_part_2};

    #[test]
    fn should_calc_fuel_example_data() {
        let mut data = parse_data("input/day_7_example_data");
        assert_eq!(37, calc_fuel(&mut data))
    }

    #[test]
    fn should_calc_fuel_part_2_example_data() {
        let mut data = parse_data("input/day_7_example_data");
        assert_eq!(168, calc_fuel_part_2(&mut data))
    }

    #[test]
    fn should_calc_fuel_part_2_test_data() {
        let mut data = parse_data("input/day_7_test_data");
        assert_eq!(92948968, calc_fuel_part_2(&mut data))
    }

    #[test]
    fn should_calc_fuel_test_data() {
        let mut data = parse_data("input/day_7_test_data");
        assert_eq!(340052, calc_fuel(&mut data))
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<u32> {
        data_parser::load_file(p)
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect()
    }
}
