mod my_lib;

use my_lib::{assert_and_print, get_input_as_string};

type Storage = Vec<Option<u16>>;

fn parse(demo_input: &bool) -> Storage {
    let file_contents = get_input_as_string(demo_input);

    let (storage, _) =
        file_contents
            .chars()
            .enumerate()
            .fold((vec![], 0_u16), |(mut res, file_id), (i, c)| {
                let is_file = i % 2 == 0;
                let num: usize = c.to_digit(10).unwrap().try_into().unwrap();

                if is_file {
                    let mut ids = vec![Some(file_id); num];

                    res.append(&mut ids);
                    return (res, file_id + 1);
                }

                let mut dots = vec![None; num];
                res.append(&mut dots);

                (res, file_id)
            });

    storage
}

fn storage_is_compact(storage: &Storage) -> bool {
    let left_most_empty_idx = storage
        .iter()
        .enumerate()
        .find(|(_, e)| e.is_none())
        .map(|(i, _)| i);

    if left_most_empty_idx.is_none() {
        return true;
    }

    let first_file_after_left_empty = storage
        .iter()
        .skip(left_most_empty_idx.unwrap())
        .find(|c| c.is_some());

    first_file_after_left_empty.is_none()
}

// TODO: this recursive function overflows its stack unless compiled with release mode
fn compact_storage(storage: &Storage) -> Storage {
    if storage_is_compact(storage) {
        return storage.clone();
    }

    let (right_most_file_idx, right_most_file_id) = storage
        .iter()
        .enumerate()
        .rfind(|(_, e)| e.is_some())
        .unwrap();

    let (left_most_empty_idx, _) = storage
        .iter()
        .enumerate()
        .find(|(_, e)| e.is_none())
        .unwrap();

    let new_storage: Storage = storage
        .iter()
        .enumerate()
        .map(|(i, e)| {
            if i == right_most_file_idx {
                return None;
            }
            if i == left_most_empty_idx {
                return *right_most_file_id;
            }
            *e
        })
        .collect();

    compact_storage(&new_storage)
}

fn calc_checksum(s: &Storage) -> usize {
    s.iter().enumerate().fold(0, |acc, (i, c)| {
        if c.is_some() {
            let file_id = c.unwrap() as usize;
            return acc + file_id * i;
        }
        acc
    })
}

fn part_one(storage: &Storage) -> usize {
    let compact = compact_storage(storage);
    calc_checksum(&compact)
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (1_928, 6_446_899_523_367), &demo_input);
}
