pub fn binary_search(search_value: i32) -> Option<usize> {
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut left: usize = 0;
    let mut right: usize = numbers.len();
    let mut _mid: usize = left + right / 2;

    if !(numbers.is_empty() || numbers.iter().is_sorted()) {
        println!("Isn't sorted");
    }

    println!("{}", "lets do the binary search!");

    while left != right {
        _mid = (left + right) / 2;
        if search_value == numbers[_mid] {
            return Some(_mid);
        } else if search_value > numbers[_mid] {
            left = _mid + 1;
            // since mid has already been checked
            // right stays as is
        } else {
            right = _mid - 1;
            // since mid has already been checked
            // left stays as is
        }
    }

    return None;
}
