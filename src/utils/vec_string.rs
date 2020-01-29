#[macro_export]
macro_rules! vec_string {
    ($($x:expr),*) => (vec![$($x.to_owned()), *]);
}

#[test]
fn test_utils_vec_of_strings() {
    let vec_ = vec_string!["a", "b", "c"];
    assert_eq!(vec_[0], "a");
    assert_eq!(vec_[1], "b");
    assert_eq!(vec_[2], "c");
}
