use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
       
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is {}", secret_number);
    
    loop {
       
      println!("Please input your guess.");
        
      let guess2: u32 = "42".parse().expect("Not a number!");
        
      let mut guess = String::new();
    
      io::stdin().read_line(&mut guess).expect("Failed to read line");
    
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => { println!("bad"); continue },
      };
      
//      let guess: u64 = guess.trim().parse().expect("Not a number!");

      println!("You guessed: {}", guess);
    
      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
    }
}

