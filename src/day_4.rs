use std::collections::HashSet;

#[derive(Default, Debug)]
struct Number {
    val: usize,
    marked: bool,
}

#[derive(Debug)]
struct Board {
    numbers: Vec<Vec<Number>>,
}

struct Bingo {
    boards: Vec<Board>,
    call_numbers: Vec<usize>,
}

impl Number {
    fn new(val: usize) -> Self {
        Number {
            val,
            ..Default::default()
        }
    }
}

impl Bingo {
    fn first_win(&mut self) -> usize {
        for i in self.call_numbers.iter() {
            for b in self.boards.iter_mut() {
                if b.any_won(*i) {
                    return b.sum_unmarked() * i;
                }
            }
        }
        unreachable!("something went wrong")
    }

    fn last_win(&mut self) -> usize {
        let mut finished = HashSet::new();
        let len = self.boards.len();
        for i in self.call_numbers.iter() {
            for (idx, b) in self.boards.iter_mut().enumerate() {
                if !finished.contains(&idx) && b.any_won(*i) {
                    finished.insert(idx);
                    if len - finished.len() == 0 {
                        return b.sum_unmarked() * i;
                    }
                }
            }
        }
        unreachable!("something went wrong")
    }
}

impl Board {
    fn new(numbers: Vec<Vec<Number>>) -> Self {
        Board { numbers }
    }

    fn print(&self) {
        println!("_ _ _ _ _ start");
        self.numbers.iter().for_each(|row| println!("{:?}", row));
        println!("_ _ _ _ _ end");
    }

    fn any_won(&mut self, c: usize) -> bool {
        let (mut x, mut y, mut found) = (0, 0, false);
        self.numbers.iter_mut().enumerate().for_each(|(i, n)| {
            n.iter_mut().enumerate().for_each(|(j, n)| {
                if n.val == c {
                    n.marked = true;
                    x = i;
                    y = j;
                    found = true;
                }
            })
        });

        if found {
            let row_won = self.numbers[x].iter().all(|i| i.marked);
            if row_won {
                return true;
            }

            let column_won = self
                .numbers
                .iter()
                .map(|r| r.get(y).unwrap())
                .all(|n| n.marked);
            if column_won {
                return true;
            }
        }

        false
    }

    fn sum_unmarked(&self) -> usize {
        self.numbers
            .iter()
            .flat_map(|n| n.iter().filter(|n| !n.marked).map(|n| n.val))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::data_parser;
    use crate::day_4::{Bingo, Board, Number};

    #[test]
    fn should_calc_first_win_board_score_example_data() {
        let mut bingo = parse_data("input/day_4_example_input");
        assert_eq!(4512, bingo.first_win());
    }

    #[test]
    fn should_calc_lst_win_board_score_example_data() {
        let mut bingo = parse_data("input/day_4_example_input");
        assert_eq!(1924, bingo.last_win());
    }

    #[test]
    fn should_calc_first_win_board_score_test_data() {
        let mut bingo = parse_data("input/day_4_test_input");
        assert_eq!(4512, bingo.first_win());
    }

    #[test]
    fn should_calc_lst_win_board_score_test_data() {
        let mut bingo = parse_data("input/day_4_test_input");
        assert_eq!(1827, bingo.last_win());
    }

    #[test]
    fn should_calc_last_win_board_score_test_data() {
        let mut bingo = parse_data("input/day_4_test_input");
        assert_eq!(4512, bingo.first_win());
    }

    fn parse_data<P: AsRef<Path>>(p: P) -> Bingo {
        let mut data = data_parser::parse_on_blank_lines(p);
        let call_numbers = data
            .remove(0)
            .split(',')
            .map(|n| n.parse().expect("Not a number"))
            .collect::<Vec<usize>>();

        let boards = data
            .iter()
            .map(|row| {
                let raw_board = row
                    .split_whitespace()
                    .map(|n| n.parse().expect("Not a number"))
                    .collect::<Vec<usize>>();
                let board = (0..=25)
                    .into_iter()
                    .step_by(5)
                    .skip(1)
                    .map(|i| {
                        let slice = &raw_board[i - 5..i];
                        slice
                            .to_owned()
                            .into_iter()
                            .map(Number::new)
                            .collect::<Vec<Number>>()
                    })
                    .collect::<Vec<Vec<Number>>>();

                Board::new(board)
            })
            .collect::<Vec<Board>>();

        Bingo {
            boards,
            call_numbers,
        }
    }
}
