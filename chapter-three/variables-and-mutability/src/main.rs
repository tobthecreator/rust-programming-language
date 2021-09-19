fn main() {
    // MUTABILITY
    let x = 5; // immutable

    println!("x = {}", x);
    // x = 6; // cannot assign twice to immutable variable
    // println!("x = {}", x);

    let mut y = 5;
    println!("y = {}", y);

    y = 6;
    println!("y = {}", y);

    // VARIABLES VS CONSTANTS
    // const mut z = 1; // cannot be mutable

    // const may only be set to a constant expression, they cannot be set by any runtime value
    const TOTAL: u32 = 100_000;

    // SHADOWING
    /*
        we can redeclare values with shadowing, changing their types

        this is different than mutability because we're functionally redeclaring
    */

    let a = 1;
    let a = a + 2;
    println!("a = {}", a);

    let a = "I'm a string now!";
    println!("a = {}", a);

    let b: i32 = 1;
    let b = b + 77;
    println!("b = {}", b);
    let b = "I'm also a string now!";
    println!("b = {}", b);

    // the above is different than if we declared mutable variable
    let mut c: i32 = 1;
    c = c + 77;
    println!("c = {}", c);
    // c = "I'm also a string now!"; // throws error on type change
    println!("c = {}", c);
}
