use std::collections::HashMap;
use std::io;

use regex::{Captures, Regex, RegexBuilder};

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

    basic_rh_interactive();
}

fn basic_rh_interactive() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::from([
        (String::from("Sales"), Vec::from([String::from("Henry"), String::from("Carlotta"), String::from("Robert")])),
        (String::from("Engineering"), Vec::from([String::from("Arthur"), String::from("Elena"), String::from("Angela"), String::from("Ricardo"), String::from("Michele")]))
    ]);

    loop {
        println!("Please input your request: ");

        let mut request = String::new();

        io::stdin()
            .read_line(&mut request)
            .expect("Failed to read line");

        println!("You asked: {request}");
        // let fn_once: Box<dyn FnOnce(&mut HashMap<String, Vec<String>>) -> ()> =
        match make_function(request) {
            Ok(fn_once) => { fn_once(&mut employees) }
            Err(_) => { println!("Error when making function, nothing to do") }
        }
        // employees.insert()

        show_employees_by_department(&employees);
    }
}

fn make_function(request: String) -> Result<Box<dyn FnOnce(&mut HashMap<String, Vec<String>>) -> ()>, String> {
    if ! check_request(&request) {
        return Err(String::from("unrecognized request (too short)"));
    }

    let re_add_result = RegexBuilder::new(r"Add +(\w+) +to +(\w+) *")
        .case_insensitive(true)
        .build();
    let re_add: Regex;
    if re_add_result.is_ok() {
        println!("re_add_result.is_ok()");
        re_add = re_add_result.unwrap();
    } else {
        return Err(String::from("unrecognized request"));
    }
    let captures_option = re_add.captures(&*request);
    let add_values: Vec<String> = captures_option.map_or_else(|| vec![], |captures: Captures| vec![captures[1].to_string(), captures[2].to_string()]);
    if add_values.len() > 0 {
        return Ok(make_closure(add_values[0].clone(), add_values[1].clone()))
    }
    return return Err(String::from("unrecognized request"));
}

fn make_closure(p0: String, p1: String) -> Box<dyn FnOnce(&mut HashMap<String, Vec<String>>) -> ()> {
    Box::new(|h| {
        let binding = Vec::from([]);
        let mut dep_list: Vec<String> = h.get(&p1).unwrap_or(&binding).to_owned();
        dep_list.push(p0);
        h.insert(p1, (*(dep_list.clone())).to_owned());
    })
}

fn check_request(request: &String) -> bool {
    request.len() > 7
}

fn show_employees_by_department(employees: &HashMap<String, Vec<String>>) {
    for department in employees.keys() {
        println!("{}", department);
        let department_employees: &Vec<String> = employees.get(department).unwrap();
        for employee in department_employees {
            println!("{}", format!("{:>50}", employee));
        }
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


