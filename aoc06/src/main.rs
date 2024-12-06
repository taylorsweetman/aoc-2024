mod my_lib;

use std::collections::HashSet;

use my_lib::{assert_and_print, get_input_as_string};

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

type CoOrd = (usize, usize);
type Parsed = (CoOrd, HashSet<CoOrd>, CoOrd);

fn parse(demo_input: &bool) -> Parsed {
    let file_content = get_input_as_string(demo_input);
    let file_lines: Vec<&str> = file_content.lines().collect();

    let max_y = file_lines.len() - 1;
    let max_x = file_lines[0].len() - 1;
    let mut guard_loc = (0_usize, 0_usize);
    let mut obstacle_locs: HashSet<CoOrd> = HashSet::new();

    file_lines.iter().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| match ch {
            '#' => {
                obstacle_locs.insert((x, y));
            }
            '^' => {
                guard_loc = (x, y);
            }
            _ => {}
        });
    });

    (guard_loc, obstacle_locs, (max_x, max_y))
}

fn rotate_guard(dir: &Dir) -> Dir {
    match dir {
        Dir::Up => Dir::Right,
        Dir::Right => Dir::Down,
        Dir::Down => Dir::Left,
        Dir::Left => Dir::Up,
    }
}

fn move_guard(dir: &Dir, loc: &CoOrd) -> CoOrd {
    let (x, y) = loc;
    match dir {
        Dir::Up => (*x, y - 1),
        Dir::Right => (x + 1, *y),
        Dir::Down => (*x, y + 1),
        Dir::Left => (x - 1, *y),
    }
}

fn guard_will_leave(guard_loc: &CoOrd, max_vals: &CoOrd, current_dir: &Dir) -> bool {
    let (guard_x, guard_y) = guard_loc;
    let (max_x, max_y) = max_vals;

    match current_dir {
        Dir::Up => *guard_y == 0,
        Dir::Right => guard_x == max_x,
        Dir::Down => guard_y == max_y,
        Dir::Left => *guard_x == 0,
    }
}

fn update_traversed_squares(
    traversed_squares: &mut HashSet<CoOrd>,
    guard_loc: &CoOrd,
    current_dir: &Dir,
    obstacle_locs: &HashSet<CoOrd>,
    max_vals: &CoOrd,
) {
    // base case
    if guard_will_leave(guard_loc, max_vals, current_dir) {
        return;
    }

    let potential_new_loc = move_guard(current_dir, guard_loc);

    // guard would hit obstacle case
    if obstacle_locs.contains(&potential_new_loc) {
        return update_traversed_squares(
            traversed_squares,
            guard_loc,
            &rotate_guard(current_dir),
            obstacle_locs,
            max_vals,
        );
    }

    // guard moves case
    traversed_squares.insert(potential_new_loc);
    update_traversed_squares(
        traversed_squares,
        &potential_new_loc,
        current_dir,
        obstacle_locs,
        max_vals,
    )
}

fn part_one(parsed: &Parsed) -> usize {
    let mut traversed_squares: HashSet<CoOrd> = HashSet::from_iter([parsed.0].iter().cloned());
    update_traversed_squares(
        &mut traversed_squares,
        &parsed.0,
        &Dir::Up,
        &parsed.1,
        &parsed.2,
    );
    traversed_squares.len()
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (41, 5331), &demo_input);
}
