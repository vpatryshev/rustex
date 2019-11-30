fn main() {

    #[derive(Debug)]
    enum IpAddr {
        V4(u8,u8,u8,u8),
        V6(String),
    }

    let _home = IpAddr::V4(127,0,0,1);

    let _loopback = IpAddr::V6(String::from("::1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self, _myself: Message) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call(Message::Write(String::from("hello")));

//    println!("home: {}", home);
//    println!("loop: {}", loopback);
}
