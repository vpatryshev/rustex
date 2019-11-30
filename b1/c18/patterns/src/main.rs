struct Point {
    x: i32,
    y: i32,
}

fn main() {

    let x = 5;

    match x {
        1...5 => println!("one through five"),
        _ => println!("something else"),
    }
    
    let p = Point { x: 0, y: 7 };

    let Point { x: x1, y } = p;
    assert_eq!(0, x1);
    assert_eq!(7, y);

    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

// Won't compile (s' content moved into _s) (then what?):    
//    if let Some(_s) = s {
//        println!("found a string {}", _s);
//    }

    println!("{:?}", s);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    match numbers {
        (.., last) => {
            println!("Last number: {}", last);
        },
    }

    let num = Some(4);
std::ops;
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}