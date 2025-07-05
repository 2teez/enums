use super::*;

#[test]
fn test_enums_on_vec() {
    let langs = vec!["c", "c++", "zig-lang", "java", "rust"];
    let got = langs.enums();
    let expected = vec![
        (0, "c"),
        (1, "c++"),
        (2, "zig-lang"),
        (3, "java"),
        (4, "rust"),
    ];
    assert_eq!(expected, got);
}

#[test]
fn test_enums_custom_start_index_on_vec() {
    let langs = vec!["c", "c++", "zig-lang", "java", "rust"];
    let got = langs.enums_start_at(2.into());
    let expected = vec![
        (2, "c"),
        (3, "c++"),
        (4, "zig-lang"),
        (5, "java"),
        (6, "rust"),
    ];
    assert_eq!(expected, got);
}

#[test]
fn test_enums_custom_start_on_slice_using_starter_struct_new() {
    let ints = [3, 8, 0, 2, 12, 76, -4];
    let got = ints.enums_start_at(Starter::new());
    let expected = vec![(0, 3), (1, 8), (2, 0), (3, 2), (4, 12), (5, 76), (6, -4)];
    assert_eq!(expected, got);
}

#[test]
fn test_enums_custom_start_on_slice_using_starter_default_function() {
    let ints = [3, 8, 0, 2, 12, 76, -4];
    let got = ints.enums_start_at(Starter::default());
    let expected = vec![(0, 3), (1, 8), (2, 0), (3, 2), (4, 12), (5, 76), (6, -4)];
    assert_eq!(expected, got);
}

#[test]
fn test_enums_custom_start_on_slice_using_starter_with_a_value() {
    let floats = [0.3, 8.15, 0.51, -2.03, 12., 7.6, -4.0015].enums_start_at(Starter(4));
    let got = (8, 12.);
    for (ind, value) in floats {
        if ind == 8 {
            assert_eq!((ind, value), got);
        }
    }
}

#[test]
#[ignore = "index still starts at zero."]
fn test_enums_get_a_custom_start_on_a_slice() {
    let nums = [1, 3, 5, 7, 2, 4, 6, 8].enums_start_at(1.into());
    let got = nums[1];
    let expected = (1, 1);
    assert_eq!(
        expected, got,
        "still started the index from zero when used the index"
    );
}

#[test]
fn test_empty_vec() {
    let empty: Vec<u8> = vec![];
    assert_eq!(empty.enums(), vec![]);
}

#[test]
fn test_single_element_vec() {
    let nums = vec![42];
    assert_eq!(nums.enums(), vec![(0, 42)]);
}

#[test]
fn test_vec_using_usize_max() {
    let nums = vec![42];
    assert_eq!(
        nums.enums_start_at(usize::MAX.into()),
        vec![(usize::MAX, 42)]
    );
}

#[test]
fn gets_returned_hashmap() {
    use std::collections::HashMap;

    let langs = vec!["c", "c++", "zig-lang", "java", "rust"];
    let got = langs.to_map();
    let mut expected: HashMap<usize, &str> = HashMap::new();
    expected.insert(0, "c");
    expected.insert(1, "c++");
    expected.insert(2, "zig-lang");
    expected.insert(3, "java");
    expected.insert(4, "rust");
    assert_eq!(expected, got);
}
