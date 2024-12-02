mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = Vec<Vec<u32>>;

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    file_contents
        .split('\n')
        .map(|s| s.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn report_is_safe(report: &Vec<u32>) -> bool {
    let mut is_increasing: Option<bool> = None;

    for i in 0..(report.len() - 1) {
        let current_level = report[i];
        let next_level = report[i + 1];

        if is_increasing.is_none() {
            is_increasing = Some(next_level > current_level);
        }

        if is_increasing.unwrap() && current_level >= next_level {
            return false;
        }

        if !is_increasing.unwrap() && next_level >= current_level {
            return false;
        }

        if next_level.abs_diff(current_level) > 3 {
            return false;
        }
    }

    true
}

fn part_one(parsed: &Parsed) -> usize {
    let safe_report_count = parsed.iter().filter(|r| report_is_safe(*r)).count();
    safe_report_count
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (2, 663), &demo_input);
}
