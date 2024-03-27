use std::collections::HashMap;

fn main() {
    let v = vec![3, 56, 234, 3, 56, 9, 2, 324, 879, 56, 456, 3, 2, 9, 9, 324, 56, 56, 3];
    find_median_and_mode(v);
}

fn find_median_and_mode(mut v: Vec<i32>) -> (i32, i32) {
    v.sort();
    (find_median(&v), find_mode(&v))
}

fn find_median(v: &Vec<i32>) -> i32 {
    let idx = (v.len() / 2) as i32;
    let median_option: Option<&i32> = v.get(idx as usize);
    let mut median: i32 = -1;
    let message: String;
    match median_option {
        Some(val) => {
            median = *val;
            message = format!("The median is : {}", median);
        },
        None => message = format!("median is None, something is wrong with the index {idx}")
    }
    println!("{}", message);

    median
}

fn find_mode(v: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();
    let mut max_found = 0;
    let mut mode = 0;
    for i in v {
        let count = counts.entry(i).or_insert(0);
        *count += 1;
        if *count > max_found {
            mode = *i;
            max_found = *count;
        }
    }

    println!("Mode is {mode}");

    mode
}
