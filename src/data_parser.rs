use std::path::Path;

pub fn parse_file<P>(path: P) -> Vec<usize>
where
    P: AsRef<Path>,
{
    parse(path, |s| {
        s.parse().expect("Not the parse you're looking for")
    })
}

pub fn parse_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    parse(path, |s| s.into())
}

pub fn parse_on_blank_lines<P>(path: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let split_reg = regex::Regex::new("\n\\s*\n").unwrap();
    split_reg
        .split(&load_file(path))
        .map(|l| l.replace("\n", " "))
        .collect()
}

pub fn parse<T, P, F>(path: P, f: F) -> Vec<T>
where
    P: AsRef<Path>,
    F: Fn(&str) -> T,
{
    load_file(path).lines().map(f).collect()
}

pub fn load_file<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    std::fs::read_to_string(path).expect("Can't load file")
}

pub fn parse_matrix<P: AsRef<Path>>(p: P) -> Vec<Vec<usize>> {
    parse_lines(p)
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}
