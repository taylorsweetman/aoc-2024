mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = Vec<(u64, Vec<u64>)>;

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    let raw_results: Vec<(&str, &str)> = file_contents
        .lines()
        .map(|s| {
            let mut split = s.split(": ");
            (split.next().unwrap(), split.next().unwrap())
        })
        .collect();

    raw_results
        .iter()
        .map(|(l, r)| {
            (
                l.parse().unwrap(),
                r.split(' ').map(|n| n.parse().unwrap()).collect(),
            )
        })
        .collect()
}

// TODO -- ugly brute force
fn find_all_possible_results(values: &[u64]) -> Vec<u64> {
    let mut results = vec![values[0]];

    values.iter().skip(1).for_each(|next_value| {
        let new_results = results
            .clone()
            .iter()
            .flat_map(|v| vec![next_value + v, next_value * v])
            .collect();
        results = new_results;
    });

    results
}

fn can_achieve_target(input: &(u64, Vec<u64>)) -> bool {
    let (target, values) = input;
    find_all_possible_results(values)
        .iter()
        .any(|r| r == target)
}

fn part_one(parsed: &Parsed) -> u64 {
    parsed
        .iter()
        .filter(|a| can_achieve_target(a))
        .map(|(t, _)| t)
        .sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (3_749, 20_281_182_715_321), &demo_input);
}
