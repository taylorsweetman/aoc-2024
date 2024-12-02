mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Parsed = Vec<Vec<u32>>;

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    file_contents
        .lines()
        .map(|s| s.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn get_failing_level_index(report: &[u32]) -> Option<usize> {
    let mut up_count = 0;
    let mut down_count = 0;

    for i in 0..(report.len() - 1) {
        let current_level = report[i];
        let next_level = report[i + 1];

        if current_level == next_level {
            return Some(i);
        }

        if next_level.abs_diff(current_level) > 3 {
            return Some(i);
        }

        if next_level > current_level {
            up_count += 1;
        }

        if current_level > next_level {
            down_count += 1;
        }

        if up_count != i + 1 && down_count != i + 1 {
            return Some(i);
        }
    }

    None
}

fn remove_at_idx(v: &[u32], idx: &usize) -> Vec<u32> {
    v.iter()
        .enumerate()
        .filter(|(i, _)| i != idx)
        .map(|(_, val)| *val)
        .collect()
}

fn report_is_safe(report: &[u32], dampen_once: bool) -> bool {
    let failing_index = get_failing_level_index(report);

    if failing_index.is_none() {
        return true;
    }

    if dampen_once {
        let failing_index_val = failing_index.unwrap();

        let report_is_safe_with_idx_removed =
            report_is_safe(&remove_at_idx(report, &failing_index_val), false);
        let report_is_safe_with_next_idx_removed =
            report_is_safe(&remove_at_idx(report, &(failing_index_val + 1)), false);

        if failing_index_val != 1 {
            return report_is_safe_with_idx_removed || report_is_safe_with_next_idx_removed;
        } else {
            let report_is_safe_with_first_level_removed =
                report_is_safe(&remove_at_idx(report, &0), false);

            return report_is_safe_with_idx_removed
                || report_is_safe_with_next_idx_removed
                || report_is_safe_with_first_level_removed;
        }
    }

    false
}

fn part_one(parsed: &Parsed) -> usize {
    let safe_report_count = parsed.iter().filter(|r| report_is_safe(r, false)).count();
    safe_report_count
}

fn part_two(parsed: &Parsed) -> usize {
    let safe_report_count = parsed.iter().filter(|r| report_is_safe(r, true)).count();
    safe_report_count
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (2, 663), &demo_input);

    let part_two_answer = part_two(&parsed);
    assert_and_print(&part_two_answer, (4, 692), &demo_input);
}
