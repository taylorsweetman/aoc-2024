mod my_lib;

use std::collections::{HashMap, HashSet};

use my_lib::{assert_and_print, get_input_as_string};

type CoOrd = (usize, usize);
type Parsed = (HashMap<char, Vec<CoOrd>>, CoOrd);

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    let lines: Vec<&str> = file_contents.lines().collect();
    let max_y = lines.len() - 1;
    let max_x = lines[0].len() - 1;

    let mut result: HashMap<char, Vec<CoOrd>> = HashMap::new();

    file_contents.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                result
                    .entry(ch)
                    .and_modify(|v| {
                        v.push((x, y));
                    })
                    .or_insert(vec![(x, y)]);
            }
        });
    });

    (result, (max_x, max_y))
}

fn find_pairs(map: &HashMap<char, Vec<CoOrd>>) -> HashSet<(CoOrd, CoOrd)> {
    map.iter().fold(HashSet::new(), |mut acc, (_, locations)| {
        for i in 0..(locations.len() - 1) {
            let current = locations[i];
            locations.iter().skip(i + 1).for_each(|next| {
                acc.insert((current, *next));
            });
        }
        acc
    })
}

fn find_anti_nodes(pair_locations: &(CoOrd, CoOrd), max_idxs: &CoOrd) -> Vec<CoOrd> {
    let (a, b) = pair_locations;
    let left_node = if a.0 <= b.0 { a } else { b };
    let right_node = if a.0 >= b.0 { a } else { b };
    let up_node = if a.1 <= b.1 { a } else { b };
    let down_node = if a.1 >= b.1 { a } else { b };

    let delta_x = a.0.abs_diff(b.0);
    let delta_y = a.1.abs_diff(b.1);

    let left_x = left_node.0.checked_sub(delta_x);
    let right_x = if right_node.0 + delta_x <= max_idxs.0 {
        Some(right_node.0 + delta_x)
    } else {
        None
    };
    let up_y = up_node.1.checked_sub(delta_y);
    let down_y = if down_node.1 + delta_y <= max_idxs.1 {
        Some(down_node.1 + delta_y)
    } else {
        None
    };

    let top_left = up_node.0 <= down_node.0;
    if top_left {
        [(left_x, up_y), (right_x, down_y)]
            .iter()
            .filter(|(x, y)| x.is_some() && y.is_some())
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .collect()
    } else {
        [(right_x, up_y), (left_x, down_y)]
            .iter()
            .filter(|(x, y)| x.is_some() && y.is_some())
            .map(|(x, y)| (x.unwrap(), y.unwrap()))
            .collect()
    }
}

fn part_one(parsed: &Parsed) -> usize {
    let (map, max_idxs) = parsed;

    let anti_nodes: HashSet<CoOrd> = find_pairs(map)
        .iter()
        .flat_map(|p| find_anti_nodes(p, max_idxs))
        .collect();

    anti_nodes.len()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (14, 354), &demo_input);
}
