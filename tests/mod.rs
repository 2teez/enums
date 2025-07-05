use enums::enums::Enums;
use enums::nums::Nums;
use std::collections::HashMap;

#[test]
fn test_to_map_function_vec() {
    let numbers = vec![23, 56, 90];
    let map = numbers.to_map();

    let expected: HashMap<usize, _> = [(0, 23), (1, 56), (2, 90)].into_iter().collect();
    assert_eq!(map, expected);
}

#[test]
fn test_to_map_function_slice() {
    let numbers = [23, 56, 90];
    let map = numbers.to_map();

    let expected: HashMap<usize, _> = [(0, 23), (1, 56), (2, 90)].into_iter().collect();
    assert_eq!(map, expected);
}

#[test]
fn test_getting_indices_for_empty_vec() {
    let got: Vec<usize> = vec![];
    let expected: Vec<usize> = vec![];
    assert_eq!(expected, got.nums(), "empty vector or slice");
}

#[test]
fn test_getting_indices_after_it_changed() {
    let got = vec!["java", "zig-lang", "python", "ruby"].nums_starting_at(5.into());
    let expected = [5, 6, 7, 8];
    assert_eq!(got, expected);
}
