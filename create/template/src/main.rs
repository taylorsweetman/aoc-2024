mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = Vec<String>;

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    file_contents.split('\n').map(|s| s.to_string()).collect()
}

fn part_one(parsed: &Parsed) -> i32 {
    println!("{:?}", parsed);
    1
}

fn main() {
    let demo_input = true;

    let parsed = parse(&demo_input);
    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, &(1, None), &demo_input);
}
