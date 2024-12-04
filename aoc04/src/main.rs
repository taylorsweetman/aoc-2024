mod my_lib;

use std::collections::{HashMap, HashSet};

use my_lib::{assert_and_print, get_input_as_string};

type CoOrd = (i32, i32);
type Parsed = (HashMap<CoOrd, char>, HashSet<CoOrd>);

fn parse(demo_input: &bool) -> Parsed {
    let file_contents = get_input_as_string(demo_input);
    let lines: Vec<&str> = file_contents.lines().collect();

    let mut parsed_map: HashMap<CoOrd, char> = HashMap::with_capacity(lines.len() * lines[0].len());
    let mut x_locations: HashSet<CoOrd> = HashSet::new();

    lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            let coord = (x as i32, y as i32);

            parsed_map.entry(coord).or_insert(ch);
            if ch == 'X' {
                x_locations.insert(coord);
            }
        });
    });

    (parsed_map, x_locations)
}

fn get_coords_from_ranges(xs: &[i32], ys: &[i32]) -> Vec<CoOrd> {
    xs.iter().zip(ys.iter()).map(|(x, y)| (*x, *y)).collect()
}

fn get_coord_lines_for_len(start: &CoOrd, len: &i32) -> Vec<Vec<CoOrd>> {
    let x = start.0;
    let y = start.1;

    let xs = vec![x; *len as usize];
    let ys = vec![y; *len as usize];

    let xs_right: Vec<i32> = (x..(x + len)).collect();
    let xs_left: Vec<i32> = ((x - len + 1)..(x + 1)).rev().collect();
    let ys_down: Vec<i32> = (y..(y + len)).collect();
    let ys_up: Vec<i32> = ((y - len + 1)..(y + 1)).rev().collect();

    let hor_right = get_coords_from_ranges(&xs_right, &ys);
    let hor_left = get_coords_from_ranges(&xs_left, &ys);
    let vert_down = get_coords_from_ranges(&xs, &ys_down);
    let vert_up = get_coords_from_ranges(&xs, &ys_up);
    let diag_up_right = get_coords_from_ranges(&xs_right, &ys_up);
    let diag_up_left = get_coords_from_ranges(&xs_left, &ys_up);
    let diag_down_right = get_coords_from_ranges(&xs_right, &ys_down);
    let diag_down_left = get_coords_from_ranges(&xs_left, &ys_down);

    vec![
        hor_right,
        hor_left,
        vert_down,
        vert_up,
        diag_up_right,
        diag_up_left,
        diag_down_right,
        diag_down_left,
    ]
}

fn find_possible_strings(coord: &CoOrd, map: &HashMap<CoOrd, char>) -> Vec<String> {
    let coord_combos = get_coord_lines_for_len(coord, &4);

    coord_combos
        .iter()
        .map(|combo| {
            combo
                .iter()
                .map(|coord| map.get(coord).unwrap_or(&'!'))
                .collect()
        })
        .filter(|s: &String| !s.contains('!'))
        .collect()
}

fn part_one(parsed: &Parsed) -> usize {
    let (map, x_locs) = parsed;

    let all_possible_strings: Vec<String> = x_locs
        .iter()
        .flat_map(|loc| {
            find_possible_strings(loc, map)
                .iter()
                .flat_map(|s| vec![s.to_owned(), s.chars().rev().collect::<String>()])
                .collect::<Vec<String>>()
        })
        .collect();

    let search_string = "XMAS";

    all_possible_strings
        .iter()
        .filter(|s| *s == search_string)
        .count()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (18, 2_390), &demo_input);
}
