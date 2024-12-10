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

fn compact_storage(storage: &Storage, right_most_file_idx: &Option<usize>) -> Storage {
    if storage_is_compact(storage) {
        return storage.clone();
    }

    let mut right_most_file_idxs: Vec<usize> = vec![];
    let max_idx = right_most_file_idx.unwrap_or(storage.len());
    for i in (0..max_idx).rev() {
        let current = storage[i];
        if current.is_none() {
            continue;
        }

        for j in (1..(i + 1)).rev() {
            let next = storage[j];
            if current == next {
                right_most_file_idxs.push(j);
            }
            if next.is_none() {
                break;
            }
        }
        break;
    }

    let right_most_file_id = storage[right_most_file_idxs[0]];
    let mut files_to_move_count = right_most_file_idxs.len();

    let new_storage: Storage = storage
        .iter()
        .enumerate()
        .map(|(i, e)| {
            if right_most_file_idxs.iter().any(|j| i == *j) {
                return None;
            }
            if storage[i].is_none() && files_to_move_count > 0 {
                files_to_move_count -= 1;
                return right_most_file_id;
            }
            *e
        })
        .collect();

    compact_storage(&new_storage, &right_most_file_idxs.iter().last().copied())
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
    let compact = compact_storage(storage, &None);
    calc_checksum(&compact)
}

fn main() {
    let demo_input = false;
    let parsed = parse(&demo_input);

    let part_one_answer = part_one(&parsed);
    assert_and_print(&part_one_answer, (1_928, 6_446_899_523_367), &demo_input);
}
