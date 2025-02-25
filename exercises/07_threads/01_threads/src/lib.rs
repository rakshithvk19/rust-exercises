// TODO: implement a multi-threaded version of the `sum` function
//  using `spawn` and `join`.
//  Given a vector of integers, split the vector into two halves and
//  sum each half in a separate thread.

// Caveat: We can't test *how* the function is implemented,
// we can only verify that it produces the correct result.
// You _could_ pass this test by just returning `v.iter().sum()`,
// but that would defeat the purpose of the exercise.
//
// Hint: you won't be able to get the spawned threads to _borrow_
// slices of the vector directly. You'll need to allocate new
// vectors for each half of the original vector. We'll see why
// this is necessary in the next exercise.

use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let midpoint_index = v.len() / 2;
    let v_first_half: Vec<i32> = v[..midpoint_index].to_vec();
    let v_second_half: Vec<i32> = v[midpoint_index..].to_vec();
    let sum: i32;

    let handle_first_half = thread::spawn(move || calc_sum(v_first_half));

    let handle_second_half = thread::spawn(move || calc_sum(v_second_half));

    let sum1 = handle_first_half.join().unwrap();
    let sum2 = handle_second_half.join().unwrap();
    sum1 + sum2
}

pub fn calc_sum(v: Vec<i32>) -> i32 {
    v.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
