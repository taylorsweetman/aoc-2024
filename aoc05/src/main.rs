mod my_lib;

use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use my_lib::{assert_and_print, get_input_as_string};

type PageOrderMap = HashMap<u16, HashSet<u16>>;
type Parsed = (PageOrderMap, Vec<Vec<u16>>);

fn parse_map(raw: &str) -> PageOrderMap {
    let tups: Vec<(u16, u16)> = raw
        .lines()
        .map(|l| {
            l.split('|')
                .map(|n| n.parse().unwrap())
                .fold((0, 0), |mut acc, c| {
                    if acc.0 == 0 {
                        acc.0 = c
                    } else {
                        acc.1 = c
                    }
                    acc
                })
        })
        .collect();

    let mut map: PageOrderMap = HashMap::with_capacity(tups.len());

    tups.iter().for_each(|(before, after)| {
        map.entry(*before)
            .and_modify(|set| {
                set.insert(*after);
            })
            .or_insert({
                let mut set = HashSet::new();
                set.insert(*after);
                set
            });
    });

    map
}

fn parse_updates(raw: &str) -> Vec<Vec<u16>> {
    raw.lines()
        .map(|l| l.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    let mut chunks = file_contents.split("\n\n");

    (
        parse_map(chunks.next().unwrap()),
        parse_updates(chunks.next().unwrap()),
    )
}

fn get_page_order(map: &PageOrderMap, a_page: &u16, b_page: &u16) -> Ordering {
    let empty_set = HashSet::new();
    let comes_before_a = map.get(a_page).unwrap_or(&empty_set);
    let comes_before_b = map.get(b_page).unwrap_or(&empty_set);

    if comes_before_a.contains(b_page) {
        return Ordering::Greater;
    }
    if comes_before_b.contains(a_page) {
        return Ordering::Less;
    }

    Ordering::Equal
}

fn in_correct_order(map: &PageOrderMap, a_page: &u16, b_page: &u16) -> bool {
    !matches!(get_page_order(map, a_page, b_page), Ordering::Less)
}

fn find_middle_element(elems: &[u16]) -> u16 {
    let middle_elem_idx = elems.len() / 2;
    elems[middle_elem_idx]
}

fn part_one(parsed: &Parsed) -> u16 {
    let (map, updates) = parsed;
    updates
        .iter()
        .filter(|u| u.is_sorted_by(|a, b| in_correct_order(map, a, b)))
        .map(|u| find_middle_element(u))
        .sum()
}

fn part_two(parsed: &Parsed) -> u16 {
    let (map, updates) = parsed;
    let mut updates = updates.clone();

    updates
        .iter_mut()
        .filter(|u| !u.is_sorted_by(|a, b| in_correct_order(map, a, b)))
        .map(|u| {
            find_middle_element({
                u.sort_by(|a, b| get_page_order(map, a, b));
                u
            })
        })
        .sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (143, 5_108), &demo_input);

    let part_two_answer = part_two(&parsed);
    assert_and_print(&part_two_answer, (123, 7_380), &demo_input);
}
