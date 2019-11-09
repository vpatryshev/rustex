fn main() {
    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);   // extend wave with another vector
    extend(&mut wave, &tail);   // extend wave with an array

    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);
    println!("{:?}", wave);
    extend(&mut wave, &wave);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0,
                          0.0, 1.0, 0.0, -1.0]);
    println!("{:?}", wave);
}

fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}