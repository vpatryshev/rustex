/*
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

 cargo run --bin ex2
*/  
fn main() {
    println!("1){}, 2){}", pig_latin("first"), pig_latin("apple"));
}

fn pig_latin(s: &str) -> String {
    match s.chars().next() {
        None => String::from("abc"),
        Some(c) => match c {
            'a' | 'e' | 'i' | 'o' | 'u' => String::from(s) + "hay",
            _ => String::from(&s[c.len_utf8()..]) + &s[0..c.len_utf8()] + "ay"
        }
    }
}