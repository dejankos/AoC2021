type Command = (String, usize);

#[derive(Default)]
struct Submarine {
    h_pos: usize,
    depth: usize,
    aim: usize,
}

impl Submarine {
    fn new() -> Self {
        Submarine {
            ..Default::default()
        }
    }

    fn fwd(&mut self, cmd_val: usize) {
        self.h_pos += cmd_val
    }

    fn up(&mut self, cmd_val: usize) {
        self.depth -= cmd_val
    }

    fn down(&mut self, cmd_val: usize) {
        self.depth += cmd_val
    }

    fn aim_down(&mut self, cmd_val: usize) {
        self.aim += cmd_val
    }

    fn aim_up(&mut self, cmd_val: usize) {
        self.aim -= cmd_val
    }

    fn aim_fwd(&mut self, cmd_val: usize) {
        self.h_pos += cmd_val;
        self.depth += cmd_val * self.aim;
    }
}

fn calculate_course(cmds: Vec<Command>, use_aim: bool) -> usize {
    let mut sub = Submarine::new();

    cmds.iter().for_each(|cmd| match cmd.0.as_str() {
        "forward" => match use_aim {
            false => sub.fwd(cmd.1),
            _ => sub.aim_fwd(cmd.1),
        },
        "up" => match use_aim {
            false => sub.up(cmd.1),
            _ => sub.aim_up(cmd.1),
        },
        "down" => match use_aim {
            false => sub.down(cmd.1),
            _ => sub.aim_down(cmd.1),
        },
        _ => unreachable!(),
    });

    sub.h_pos * sub.depth
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;

    use super::*;

    #[test]
    fn should_calc_course_sum_example_data() {
        let data = parse_data("input/day_2_example_data.txt");
        assert_eq!(150, calculate_course(data, false));
    }

    #[test]
    fn should_calc_course_sum_test_data() {
        let data = parse_data("input/day_2_data.txt");
        assert_eq!(1654760, calculate_course(data, false));
    }

    #[test]
    fn should_calc_course_sum_example_data_part_2() {
        let data = parse_data("input/day_2_example_data.txt");
        assert_eq!(900, calculate_course(data, true));
    }

    #[test]
    fn should_calc_course_sum_test_data_part_2() {
        let data = parse_data("input/day_2_data.txt");
        assert_eq!(1956047400, calculate_course(data, true));
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Vec<Command> {
        data_parser::parse(p, |line| {
            let cmd = line.split_whitespace().collect::<Vec<&str>>();
            (cmd[0].into(), cmd[1].parse::<usize>().unwrap())
        })
    }
}
