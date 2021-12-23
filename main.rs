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
use std::collections::HashMap;

fn main() {
    let vec = vec![1, 5, 6, 3, 4, 4];
    println!("{:?}", mean_median_mode(&vec));
    let mut words = vec![
        String::from("convert"), 
        String::from("strings"),
        String::from("to"),
        String::from("pig"),
        String::from("latin"),
        String::from("from"),
        String::from("english")];
    pig_latin(&mut words);
    println!("{:?}", words);
    let mut assignments = DepartmentAssignments {
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
}

struct DepartmentAssignments {
    dept_to_names: HashMap<String, Vec<String>>,
}

impl DepartmentAssignments {
    fn add(&mut self, name: String, department: String) {
        let names_vec = self.dept_to_names.entry(department).or_insert(Vec::new());
        names_vec.push(name);
    }

    fn get_by_department(&self, department: String) -> Vec<String> {
       let vec_ref_opt = self.dept_to_names.get(&department);
       let mut new_vec = vec_ref_opt.unwrap().to_vec();
       new_vec.sort();
       new_vec
    }

    /*fn get_all_by_department() -> HashMap<String, Vec<String>> {

    }*/
}

fn is_vowel(c: &char) -> bool {
    match *c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

fn pig_latin(words: &mut Vec<String>) {
    for word in words {
        if is_vowel(&(*word).chars().next().unwrap()) {
            (*word).push_str("-ay");
        } else {
            let first_letter = (*word).chars().next().unwrap();
            (*word).remove(0);
            (*word).push_str("-");
            (*word).push_str(&first_letter.to_string());
            (*word).push_str("ay");
        }
    }
}

fn mean_median_mode(integers: &Vec<u64>) -> (f32, u64, u64) {
    
    let mut total: f32 = 0.0;
    for i in integers {
        total += *i as f32;
    }
    let len: f32 = integers.len() as f32;
    let mean = total/len;
    
    let mut new_vec = integers.to_vec();
    new_vec.sort();
    let median = 
        if new_vec.len() % 2 == 1 {
            new_vec[new_vec.len()/2]
        } else {
            (new_vec[new_vec.len()/2 - 1] + new_vec[new_vec.len()/2]) / 2
        };
    
    let mut count_map: HashMap<u64, u64> = HashMap::new();
    for i in integers {
        let count = count_map.entry(*i).or_insert(0);
        *count += 1;
    }
    
    let mut largest_count_so_far = 0;
    let mut mode = integers[0];
    for (integer, count) in &count_map {
        if *count > largest_count_so_far {
            largest_count_so_far = *count;
            mode = *integer;
        }
    }

    (mean, median ,mode)
}
