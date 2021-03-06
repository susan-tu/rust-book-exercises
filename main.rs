// (1) Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position), and mode (the value 
// that occurs most often; a hash map will be helpful here) of the list.
// 
// (2) Convert strings to pig latin. The first consonant of each word is moved to the
// end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start
// with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). 
// Keep in mind the details about UTF-8 encoding!
//
// (3) Using a hash map and vectors, create a text interface to allow a user to add employee
// names to a department in a company. For example, “Add Sally to Engineering” 
// or “Add Amir to Sales.” Then let the user retrieve a list of all people in a 
// department or all people in the company by department, sorted alphabetically.
mod lib;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let vec = vec![1, 5, 6, 3, 4, 4];
    println!("{:?}", lib::mean_median_mode(&vec).expect("Unable to calculate mean/median/mode"));
    let mut words = vec![
        String::from("convert"), 
        String::from("strings"),
        String::from("to"),
        String::from("pig"),
        String::from("latin"),
        String::from("from"),
        String::from("english")];
    lib::pig_latin(&mut words);
    println!("{:?}", words);
    let mut assignments = lib::DepartmentAssignments {
        dept_to_names: HashMap::new()
    };
    assignments.add(String::from("Mark"), String::from("Engineering"));
    assignments.add(String::from("Max"), String::from("Engineering"));
    assignments.add(String::from("Evaline"), String::from("Engineering"));
    assignments.add(String::from("Cameron"), String::from("Product"));
    assignments.add(String::from("Karen"), String::from("Finance"));
    println!("{:?}", assignments.get_by_department(String::from("Engineering")));
    println!("{:?}", assignments.get_by_department(String::from("Product")));
    println!("{:?}", assignments.get_by_department(String::from("Finance")));
    println!("{:?}", assignments.get_all_by_department());
    intentional_deadlock();
}

fn intentional_deadlock() {
    let counter_a = Arc::new(Mutex::new(0));
    let counter_b = Arc::new(Mutex::new(0));
    let counter_a1 = Arc::clone(&counter_a);
    let counter_a2 = Arc::clone(&counter_a);
    let counter_b1 = Arc::clone(&counter_b);
    let counter_b2 = Arc::clone(&counter_b);
    let handle1 = thread::spawn(move || {
        let mut num_a = counter_a1.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        {
            let mut num_b = counter_b1.lock().unwrap();
            *num_b += *num_a;

        }
        *num_a += 1;
    });

    let handle2 = thread::spawn(move || {
        let mut num_b = counter_b2.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        {
            let mut num_a = counter_a2.lock().unwrap();
            *num_a += *num_b;

        }
        *num_b += 1;
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}