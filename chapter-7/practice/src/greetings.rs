use crate::people::People;

pub fn greet(person: People) {
    match person {
        People::Alec => println!("Hello Alec!"),
        People::Tyler => println!("Hello Tyler!"),
        _ => println!("Hello stranger!"),
    }
}
