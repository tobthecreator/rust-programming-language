use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // let declares the variable, which by default is mutable
    // mut allows us to mutate it without redeclaring let

    /*
        let apples = 5;
        let apples = 6; is fair game since we're reusing let.

        let bananas = 6;
        bananas = 7; would fail.
        it needs to be

        let mut oranges = 7;
        oranges = 8;
    */
    let secret_number = rand::thread_rng().gen_range(1..=100);
    /*
        Result enums -> Ok and Err is returned
        io::Result

        Ok indicates that the op succeeded
        Err indicates that the op failed, and Err contains info as to why

        io::Result has an expect method that can be called.Result

        If you don't called .expect() the compiler will throw a warning
    */

    println!("{}", secret_number);

    loop {
        let mut guess = String::new(); // ::new() is calling an "associated function" on the type String

        println!("Guess a number!");
        io::stdin()
            .read_line(&mut guess) // references (&) are also mutable
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // match expressions branch on a pattern
        // they take scrutinee expressions, which is the value to compare the pattern to
        /*
            let x = 1;

            match x {
                1 => println!("one"),
                2 => println!("two"),
                3 => println!("three"),
                4 => println!("four"),
                5 => println!("five"),
                _ => println!("something else"),
        }
                */
        // below, we're getting back an Ordering enum, which has
        // Less, Greater and Equal.  We can then write functions on how to respond
        // to each of these cases
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smol"),
            Ordering::Greater => println!("2 big"),
            Ordering::Equal => {
                println!("Hey u got it");
                break;
            }
        }
    }
}
