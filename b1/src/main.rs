fn main() {
    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word(&my_string[..]);
    
    println!("1) {}", word);

    let my_string_literal = "hello world";

    let mut x = &my_string_literal;
    let _y = &mut x;
    
    // first_word works on slices of string literals
    let word2 = &my_string_literal[1..5];
//    word2.make_ascii_uppercase();
    let word3 = &my_string_literal[6..11];
    println!("2) {}", word2);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    println!("3) {}", word3);

}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}