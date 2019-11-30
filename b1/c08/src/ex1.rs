/*
Given a list of integers, use a vector and return the mean (the average value), median (when sorted, the value in the middle position), and mode (the value that occurs most often; a hash map will be helpful here) of the list.

 cargo run --bin ex1
*/
fn main() {

    let mut v: Vec<i32> = vec![90, 1, 2, 45, 2, 2, 45, 42, 1];
    let n = v.len();
    v.sort();
    let median: i32 = v[n/2];
    
    let (m, maxfreq) = mode(v.clone());
    
    println!("Average={}, Median={}, Mode={}({})", to_string(avg(v)), median, m, maxfreq);
}

use std::fmt::Display;

fn to_string<T: Display>(opt: Option<T>) -> String {
    opt.map(|x| format!("{}", x)).unwrap_or(String::from("N/A"))
}


fn avg(v: Vec<i32>) -> Option<f64> {
    let n = v.len();
    
    if n == 0 { None } else {
        let mut s : f64 = 0.0;

        for x in &v {
            s += *x as f64;
        }
        Some(s/((n as i32) as f64))
    }
}

fn mode(v: Vec<i32>) -> (i32, usize) {
    use std::collections::HashMap;

    let mut freq = HashMap::new();

    for x in &v {
        let count = freq.entry(x).or_insert(0);
        *count += 1;
    }

    let mut maxfreq = 0;
    let mut mode: i32 = v[0];
    for (k, v) in freq {
        if v > maxfreq {
            maxfreq = v;
            mode = *k;
        }
    }
    (mode, maxfreq)
}
