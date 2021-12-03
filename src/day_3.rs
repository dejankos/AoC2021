use std::usize;

fn calc_power_cons(diag: &[Vec<char>]) -> usize {
    let gamma_rate_raw = calc_gamma_rate(diag);
    let gamma_rate = binary_str_to_decimal(&gamma_rate_raw);

    gamma_rate * flip_bits(gamma_rate, gamma_rate_raw.len())
}

fn calc_life_support(diag: &[Vec<char>]) -> usize {
    let ox_rate = binary_str_to_decimal(&calc_rating(diag, most_common_oxygen));
    let scrub_rate = binary_str_to_decimal(&calc_rating(diag, most_common_scrubber));

    scrub_rate * ox_rate
}

fn calc_gamma_rate(diag: &[Vec<char>]) -> String {
    let bits_len = diag[0].len();

    (0..bits_len)
        .into_iter()
        .map(|i| most_common_part_1(diag, i))
        .collect::<String>()
}

fn binary_str_to_decimal(str: &str) -> usize {
    isize::from_str_radix(str, 2).unwrap() as usize
}

fn flip_bits(i: usize, k: usize) -> usize {
    let mask = (1 << k) - 1;
    !i & mask
}

fn most_common_part_1(diag: &[Vec<char>], pos: usize) -> String {
    let (z, o) = count_occurrences(diag, pos);

    if z > o {
        "0".into()
    } else {
        "1".into()
    }
}

fn calc_rating<F>(diag: &[Vec<char>], common: F) -> String
where
    F: Fn(&[Vec<char>], usize) -> String,
{
    let bits_len = diag[0].len();

    let mut data = diag.to_owned();
    for i in 0..bits_len {
        if data.len() == 1 {
            break;
        }
        let common = common(&data, i);

        data = data
            .into_iter()
            .filter(|chars| chars[i].to_string() == common)
            .collect()
    }

    data[0].iter().collect::<String>()
}

fn most_common_oxygen(diag: &[Vec<char>], pos: usize) -> String {
    let (z, o) = count_occurrences(diag, pos);

    if o >= z {
        "1".into()
    } else {
        "0".into()
    }
}

fn most_common_scrubber(diag: &[Vec<char>], pos: usize) -> String {
    let (z, o) = count_occurrences(diag, pos);

    if z <= o {
        "0".into()
    } else {
        "1".into()
    }
}

fn count_occurrences(diag: &[Vec<char>], pos: usize) -> (usize, usize) {
    let (mut z, mut o) = (0, 0);

    diag.iter().map(|chars| chars[pos]).for_each(|c| match c {
        '0' => z += 1,
        '1' => o += 1,
        _ => unreachable!("who dat"),
    });

    (z, o)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_3::{
        binary_str_to_decimal, calc_gamma_rate, calc_life_support, calc_power_cons, calc_rating,
        flip_bits, most_common_oxygen, most_common_scrubber,
    };

    #[test]
    fn should_calc_course_sum_example_data() {
        let data = parse_data("input/day_3_example_data.txt");
        let gamma_rate_s = calc_gamma_rate(&data);
        let gamma_rate = binary_str_to_decimal(&gamma_rate_s);
        assert_eq!(22, gamma_rate);
        let flipped = flip_bits(gamma_rate, gamma_rate_s.len());
        assert_eq!(9, flipped);
        assert_eq!(198, calc_power_cons(&data))
    }

    #[test]
    fn should_calc_calc_oxygen_gen_rating() {
        let data = parse_data("input/day_3_example_data.txt");
        let rating = calc_rating(&data, most_common_oxygen);
        assert_eq!("10111", &rating);
        let scrubber = calc_rating(&data, most_common_scrubber);
        assert_eq!("01010", &scrubber);
    }

    #[test]
    fn should_calc_power_test_data() {
        let data = parse_data("input/day_3_data.txt");
        assert_eq!(4006064, calc_power_cons(&data))
    }

    #[test]
    fn should_calc_life_support() {
        let data = parse_data("input/day_3_data.txt");
        assert_eq!(5941884, calc_life_support(&data))
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<Vec<char>> {
        data_parser::parse_lines(p)
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}
