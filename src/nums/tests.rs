use super::*;

#[test]
fn test_indexes_for_a_list() {
    let lists = vec![4, 2, 0];
    assert_eq!(lists.nums(), [0, 1, 2])
}

#[test]
fn test_indexes_for_a_list_using_customized_index() {
    let lists = vec!["cat", "lion", "bear"];
    assert_eq!(
        lists.nums_starting_at(5.into()),
        [5, 6, 7],
        "should be equal."
    )
}

#[test]
fn test_tuple_first_value() {
    let lists = vec!["cat", "lion", "bear"].enums();
    assert_eq!(lists[0].0, lists[0].first())
}

#[test]
fn test_tuple_second_value() {
    let lists = vec!["cat", "lion", "bear"].enums();
    assert_eq!(lists[2].1, lists[2].second())
}
