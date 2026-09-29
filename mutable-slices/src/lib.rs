pub fn transform_even_odd(slice: &mut [i32]) -> Vec<i32> {
    let mut results = Vec::new();

    for number in slice.iter_mut() {
        if *number % 2 == 0 {
            *number *= 2;
        } else {
            *number -= 1;
        }

        results.push(*number);
    }

    results
}
