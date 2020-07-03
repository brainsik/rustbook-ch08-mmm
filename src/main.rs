// mmm - Mean, Median, Mode
//
// Given a list of integers, use a vector and return the mean (the average
// value), median (when sorted, the value in the middle position), and mode (the
// value that occurs most often; a hash map will be helpful here) of the list.
//
// https://doc.rust-lang.org/stable/book/ch08-03-hash-maps.html#summary

use std::collections::HashMap;

fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2];

    // mean
    let mut sum = 0;
    for i in &list {
        sum += i;
    }
    let mean = sum / list.len();

    // median
    list.sort();
    let median = list[(list.len() + 1) / 2 - 1];

    // mode
    // This should really be a function. I tried, but ran into a problem about
    // lifetimes (which I haven't learned about yet) after wrestling with the
    // borrower (so might be doing something wrong). Moved into main to get
    // back to trying to solve the problem with the bits and pieces I know.
    let mut counts = HashMap::new();
    for num in list.iter() {
        counts.entry(num).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut mode: Vec<&usize> = Vec::new();
    if let Some(maxval) = counts.values().max() {
        for (k, v) in counts.iter() {
            assert!(v <= maxval);
            if v == maxval {
                mode.push(k);
            }
        }
    }
    if mode.len() == list.len() {
        // no mode - all values occur equally
        mode.clear();
    }

    println!("{:?} {:?} {:?}", mean, median, mode);
}
