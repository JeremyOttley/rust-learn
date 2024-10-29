fn do_something_with_array(array: &[u8]) -> u8 {
    assert!(array.len >= 5);
    array[0] + array[1] + array[2] + array[3] + array[4] + array[5]
}
