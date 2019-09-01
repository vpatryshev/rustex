/*
Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

 cargo run --bin ex3
*/
use std::collections::HashMap;

fn main() {
    let mut company: HashMap<String, Vec<String>> = HashMap::new();
    company = add(company, "Engineering", "Sally");
    company = add(company, "Engineering", "Harry");
    company = add(company, "Sales", "Amir");
    company = add(company, "Sales", "Hassan");
    let list = list_all(company);
    for dept in list {
        println!("Department: {}", dept.0);
        for (i, emp) in dept.1.iter().enumerate() {
            println!("  {}. {}", i+1, emp);
        }
    }
    
}

fn add(mut company: HashMap<String, Vec<String>>, dept: &str, employee: &str) -> HashMap<String, Vec<String>> {
    let list = company.entry(String::from(dept)).or_insert(Vec::new());
    list.push(String::from(employee));
    company
}

fn list_all(company: HashMap<String, Vec<String>>) -> Vec<(String, Vec<String>)> {
    let mut list: Vec<(String, Vec<String>)> = Vec::new();

    for (k, v) in company {
        let mut vc = v.clone();
        vc.sort();
        list.push((k, vc));
    }
    
    list.sort_by(|a,b| b.0.cmp(&a.0));
    list
}