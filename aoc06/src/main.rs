mod my_lib;

use std::collections::{HashMap, HashSet};

use my_lib::{assert_and_print, get_input_as_string};

#[derive(Clone, PartialEq)]
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

fn guard_loops(
    traversed_squares: &mut HashMap<CoOrd, Dir>,
    guard_loc: &CoOrd,
    current_dir: &Dir,
    obstacle_locs: &HashSet<CoOrd>,
    max_vals: &CoOrd,
) -> bool {
    // base case, guard exits
    if guard_will_leave(guard_loc, max_vals, current_dir) {
        return false;
    }

    let potential_new_loc = move_guard(current_dir, guard_loc);

    // base case, guard has already visited this square in this direction
    if traversed_squares.get(&potential_new_loc) == Some(current_dir) {
        return true;
    }

    // guard would hit obstacle case
    if obstacle_locs.contains(&potential_new_loc) {
        return guard_loops(
            traversed_squares,
            guard_loc,
            &rotate_guard(current_dir),
            obstacle_locs,
            max_vals,
        );
    }

    // guard moves case
    traversed_squares.insert(potential_new_loc, current_dir.clone());
    guard_loops(
        traversed_squares,
        &potential_new_loc,
        current_dir,
        obstacle_locs,
        max_vals,
    )
}

fn part_one(parsed: &Parsed) -> usize {
    let mut traversed_squares: HashMap<CoOrd, Dir> =
        [(parsed.0, Dir::Up)].iter().cloned().collect();
    guard_loops(
        &mut traversed_squares,
        &parsed.0,
        &Dir::Up,
        &parsed.1,
        &parsed.2,
    );
    traversed_squares.len()
}

// BRUTE FORCE ALERT -- don't judge me, I'll learn graphs one day...
fn part_two(parsed: &Parsed) -> usize {
    let (guard_loc, obstacle_locs, max_vals) = parsed;
    let (max_x, max_y) = max_vals;
    let mut count = 0_usize;

    for x in 0..(max_x + 1) {
        for y in 0..(max_y + 1) {
            let new_obstacle_loc = (x, y);

            if !obstacle_locs.contains(&new_obstacle_loc) {
                let mut traversed_squares: HashMap<CoOrd, Dir> =
                    [(parsed.0, Dir::Up)].iter().cloned().collect();
                let mut obstacle_locs = obstacle_locs.clone();
                obstacle_locs.insert(new_obstacle_loc);

                if guard_loops(
                    &mut traversed_squares,
                    guard_loc,
                    &Dir::Up,
                    &obstacle_locs,
                    max_vals,
                ) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (41, 5_331), &demo_input);

    let part_two_answer = part_two(&parsed);
    assert_and_print(&part_two_answer, (6, 1_812), &demo_input);
}
