mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};
use regex::Regex;

type Parsed = Vec<(u32, u32)>;

fn parse(demo_input: &bool, only_active: &bool) -> Parsed {
    let file_lines: Vec<String> = get_input_as_string(demo_input)
        .lines()
        .map(|l| l.to_owned())
        .collect();
    let single_line_content = file_lines.join("");

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut haystack = single_line_content;

    if *only_active {
        let active_section_re = Regex::new(r"(^|do\(\)).*?(don't\(\)|$)").unwrap();

        let active_sections = active_section_re
            .captures_iter(&haystack)
            .map(|c| {
                let (active_section, [_, _]) = c.extract();
                active_section
            })
            .collect::<Vec<&str>>()
            .join("");

        haystack = active_sections;
    }

    mul_re
        .captures_iter(&haystack)
        .map(|c| {
            let (_, [raw_a, raw_b]) = c.extract();
            let a = raw_a.parse().unwrap();
            let b = raw_b.parse().unwrap();
            (a, b)
        })
        .collect()
}

fn sum_products(p: &Parsed) -> u32 {
    p.iter().map(|(a, b)| a * b).sum()
}

fn part_one(demo_input: &bool) -> u32 {
    sum_products(&parse(demo_input, &false))
}

fn part_two(demo_input: &bool) -> u32 {
    sum_products(&parse(demo_input, &true))
}

fn main() {
    let demo_input = false;

    let part_one_answer = part_one(&demo_input);
    assert_and_print(&part_one_answer, (161, 166_357_705), &demo_input);

    let part_two_answer = part_two(&demo_input);
    assert_and_print(&part_two_answer, (48, 88_811_886), &demo_input);
}
