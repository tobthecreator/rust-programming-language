enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Texas,
    Kansas,
}

fn value_in_cents(coin: Coin) -> u8 {
    // match allows us to compare a value
    // against a series of patterns
    // and execute code based on a pattern match
    match coin {
        // Each choice is an "arm"
        // Arms = pattern + code
        // Coin::Penny = pattern
        // => some code
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Patterns can allow us to extract values
        // out of enums
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let quarter = Coin::Quarter(UsState::Texas);

    value_in_cents(quarter);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five: {:?}", five);
    println!("six: {:?}", six);
    println!("none: {:?}", none);

    // Matches are exhaustive if yo udo not cover all cases
    // default cases can be set by _
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // unit, ~ do nothing
    }
}
