mod my_lib;

use std::collections::HashMap;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = (Vec<i32>, Vec<i32>);

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    file_contents
        .split('\n')
        .map(|line| {
            let nums: Vec<i32> = line.split("   ").map(|s| s.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect()
}

fn sort_tuple_entries(p: &Parsed) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = p;
    let mut left_sorted = left.clone();
    let mut right_sorted = right.clone();

    left_sorted.sort();
    right_sorted.sort();

    (left_sorted, right_sorted)
}

fn part_one(parsed: &Parsed) -> i32 {
    let (left_sorted, right_sorted) = sort_tuple_entries(parsed);

    left_sorted
        .iter()
        .zip(right_sorted)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn part_two(parsed: &Parsed) -> i32 {
    let (left, right) = parsed;

    let right_count_map: HashMap<i32, i32> =
        right.iter().fold(HashMap::new(), |mut acc, current| {
            acc.entry(*current)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            acc
        });

    left.iter()
        .map(|l| {
            let right_count = right_count_map.get(l).unwrap_or(&0);
            l * right_count
        })
        .sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (11, 2_815_556), &demo_input);

    let part_two_answer = part_two(&parsed);
    assert_and_print(&part_two_answer, (31, 23_927_637), &demo_input);
}
