use std::collections::HashMap;

fn main() {
    let v = vec![3, 56, 234, 3, 56, 9, 2, 324, 879, 56, 456, 3, 2, 9, 9, 324, 56, 56, 3, 345, 456, 567, 567, 678, 789, 123, 123, 345, 234];
    find_median_and_mode(v);

    let v: Vec<&str> = vec![
        "first",
        "apple"
    ];
    for word in v {
        println!("{}", convert_to_pig_latin(word));
    }
}

fn convert_to_pig_latin(word: &str) -> String {
    if word.len() == 0 {
        String::from("")
    } else {
        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let first_char: char = word.chars().next().unwrap();
        if VOWELS.contains(&first_char) {
            String::from(word) + "-hay"
        } else {
            String::from(&word[1..]) + "-" + first_char.to_string().as_str() + "ay"
        }
    }
}

fn find_median_and_mode(mut v: Vec<i32>) -> (i32, i32) {
    v.sort();
    (find_median(&v), find_mode(&v))
}

fn find_median(v: &Vec<i32>) -> i32 {
    let mut median: i32 = -1;
    let message: String;
    if !v.is_empty() {
        let mut v = v.clone();
        v.dedup();
        let idx = v.len() / 2;
        match v.get(idx) {
            Some(val) => {
                median = *val;
                message = format!("The median is : {}", median);
            },
            None => message = format!("median is None, something is wrong with the index {idx}")
        }
    } else {
        message = String::from("Vector is empty…")
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


