use std::usize;

fn calc_power_cons(diag: &[Vec<char>]) -> usize {
    let gamma_rate_raw = calc_gamma_rate(diag);
    let gamma_rate = binary_str_to_decimal(&gamma_rate_raw);

    gamma_rate * flip_bits(gamma_rate, gamma_rate_raw.len())
}

fn calc_gamma_rate(diag: &[Vec<char>]) -> String {
    let bits_len = diag[0].len();

    (0..bits_len)
        .into_iter()
        .map(|i| most_common(diag, i))
        .collect::<String>()
}

fn binary_str_to_decimal(str: &str) -> usize {
    isize::from_str_radix(str, 2).unwrap() as usize
}

fn flip_bits(i: usize, k: usize) -> usize {
    let mask = (1 << k) - 1;
    !i & mask
}

fn most_common(diag: &[Vec<char>], pos: usize) -> String {
    let (mut z, mut o) = (0, 0);

    diag.iter().map(|chars| chars[pos]).for_each(|c| match c {
        '0' => z += 1,
        '1' => o += 1,
        _ => unreachable!("who dat"),
    });

    if z > o {
        "0".into()
    } else {
        "1".into()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_3::{binary_str_to_decimal, calc_gamma_rate, calc_power_cons, flip_bits};

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
    fn should_calc_power_test_data() {
        let data = parse_data("input/day_3_data.txt");
        assert_eq!(4006064, calc_power_cons(&data))
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<Vec<char>> {
        data_parser::parse_lines(p)
            .into_iter()
            .map(|line| line.chars().collect())
            .collect()
    }
}
