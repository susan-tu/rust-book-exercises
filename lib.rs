use std::collections::HashMap;

pub struct DepartmentAssignments {
    pub dept_to_names: HashMap<String, Vec<String>>,
}

impl DepartmentAssignments {
    pub fn add(&mut self, name: String, department: String) {
        let names_vec = self.dept_to_names.entry(department).or_insert(Vec::new());
        names_vec.push(name);
    }

    pub fn get_by_department(&self, department: String) -> Vec<String> {
       let vec_ref_opt = self.dept_to_names.get(&department);
       let mut new_vec = vec_ref_opt.unwrap().to_vec();
       new_vec.sort();
       new_vec
    }

    pub fn get_all_by_department(&self) -> Vec<(String, Vec<String>)> {
        let mut new_vec: Vec<(String, Vec<String>)> = Vec::new();
        for (dept, names) in self.dept_to_names.iter() {
            new_vec.push((dept.clone(), names.to_vec()));
        }
        new_vec
    }
}

fn is_vowel(c: &char) -> bool {
    match *c {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

pub fn pig_latin(words: &mut Vec<String>) {
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

pub fn mean_median_mode(integers: &Vec<u64>) -> Option<(f32, u64, u64)> {
    
    if integers.len() == 0 {
        return None
    }

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

    Some((mean, median ,mode))
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn it_calculates_mean_median_mode() {
        let vec = vec![1, 5, 6, 3, 4, 4];
        let (mean, median, mode) = mean_median_mode(&vec).expect("Unable to calculate mean/median/mode");
        relative_eq!(mean, 3.83333, epsilon = f32::EPSILON);
        assert_eq!(median, 4);
        assert_eq!(mode, 4);
    }

    #[test]
    fn it_calculates_mean_median_mode_of_single_elem_list() {
        let vec = vec![2];
        let (mean, median, mode) = mean_median_mode(&vec).expect("Unable to calculate mean/median/mode");
        relative_eq!(mean, 2.0, epsilon = f32::EPSILON);
        assert_eq!(median, 2);
        assert_eq!(mode, 2);
    }

    #[test]
    fn it_calculates_mean_median_mode_of_empty_list() {
        let vec = vec![];
        assert_eq!(mean_median_mode(&vec).is_none(), true);
    }
}