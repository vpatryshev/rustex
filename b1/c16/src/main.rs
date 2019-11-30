use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use std::sync::Mutex;

fn main() {
    let handle1 = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle1.join().unwrap();

    let v = vec![1, 2, 3];

    let handle2 = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        drop(v)
    });

    handle2.join().unwrap();

    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

//    thread::spawn(move || {
//        let val = String::from("hi");
//        tx.send(val).unwrap();
//    });
//    let received = rx.recv().unwrap();
//    println!("Got: {}", received);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    for received in rx {
        println!("Got: {}", received);
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    
    {
        let mut num = m.lock().unwrap();
        *num = 7;
    }

    println!("m = {:?}", m);

}