mod my_lib;

use std::collections::{HashMap, HashSet};

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

fn is_valid_update(map: &PageOrderMap, update: &[u16]) -> bool {
    let mut seen_pages: HashSet<u16> = HashSet::with_capacity(update.len());

    // TODO: this executes longer than required once it find a false condition
    update.iter().fold(true, |acc, page| {
        let new_set = HashSet::new();
        let cannot_appear_yet = map.get(page).unwrap_or(&new_set);
        if seen_pages.intersection(cannot_appear_yet).count() != 0 {
            return false;
        }

        seen_pages.insert(*page);

        acc
    })
}

fn find_middle_element(elems: &[u16]) -> u16 {
    let middle_elem_idx = elems.len() / 2;
    elems[middle_elem_idx]
}

fn part_one(parsed: &Parsed) -> u16 {
    let (map, updates) = parsed;
    updates
        .iter()
        .filter(|u| is_valid_update(map, u))
        .map(|u| find_middle_element(u))
        .sum()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (143, 5_108), &demo_input);
}
