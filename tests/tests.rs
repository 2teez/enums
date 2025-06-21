use ::enums::nums::Nums;

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
