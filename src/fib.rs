pub fn fib(mut max: u64) {
    let mut entries_in_fib: Vec<u64> = [0, 1].to_vec();

    let mut _current: u64 = 1;
    let mut _previous: u64 = 0;

    let mut _temp: u64;

    if max == 1 {
        println!("{}", entries_in_fib[0]);
    } else if max == 2 {
        println!("{}", entries_in_fib[0]);
        println!("{}", entries_in_fib[1]);
    } else {
        while max != 0 {
            _temp = _current + _previous;
            entries_in_fib.push(_temp);

            _previous = _current;
            _current = _temp;

            max -= 1;
        }
    }

    // print all values in the vector named "entries_in_fib"

    for i in entries_in_fib.iter() {
        println!("{}", i);
    }
}
