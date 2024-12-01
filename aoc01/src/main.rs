mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = (Vec<i32>, Vec<i32>);

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    file_contents
        .split('\n')
        .map(|line| {
            let nums: Vec<i32> = line.split("   ").map(|s| s.parse().unwrap()).collect();
            return (nums[0], nums[1]);
        })
        .collect()
}

fn part_one(parsed: &Parsed) -> i32 {
    let (left, right) = parsed;
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();
    left_sorted.sort();
    right_sorted.sort();

    left_sorted
        .iter()
        .zip(right_sorted)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (11, 2815556), &demo_input);
}
