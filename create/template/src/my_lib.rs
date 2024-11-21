use std::{fmt::Debug, fs};

pub fn get_input_as_string(demo_input: &bool) -> String {
    let path = if *demo_input { "demo.txt" } else { "input.txt" };
    fs::read_to_string(path).expect("Something went wrong reading the file")
}

pub fn assert_and_print<T>(answer: &T, expected_vals: &(T, Option<T>), demo_input: &bool) -> ()
where
    T: Debug,
    T: PartialEq,
{
    let expected = if *demo_input {
        &expected_vals.0
    } else {
        expected_vals.1.as_ref().unwrap()
    };

    assert!(
        *answer == *expected,
        "expected: {:?} got: {:?}",
        expected,
        answer
    );
    println!("answer: {:?}", answer);
}
