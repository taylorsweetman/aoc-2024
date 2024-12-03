mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};
use regex::Regex;

type Parsed = Vec<(u32, u32)>;

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    re.captures_iter(&file_contents)
        .map(|c| {
            let (_, [raw_a, raw_b]) = c.extract();
            let a = raw_a.parse().unwrap();
            let b = raw_b.parse().unwrap();
            (a, b)
        })
        .collect()
}

fn part_one(parsed: &Parsed) -> u32 {
    parsed.iter().map(|(a, b)| a * b).sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (161, 166_357_705), &demo_input);
}
