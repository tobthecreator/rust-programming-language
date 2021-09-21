fn main() {
    // Option
    // defined by the standard lib
    // defines a scenario where a value could be something or nothing

    /*
        enum Option<T> {
            None,
            Some(T),
        }
    */

    let a_number = Some(5);
    let a_string = Some("a string");
    println!("{:?}", a_number);
    println!("{:?}", a_string);

    // We use None instead of null
    // And None has to infer the type it should be if it existed
    let no_number: Option<i32> = None;
    let no_string: Option<i32> = None;
    println!("{:?}", no_number);
    println!("{:?}", no_string);

    // Option<T> needs to conver to T
    let a: i32 = 32;
    let b: Option<i32> = Some(32);

    // These are differnet types
    let c = a + b;

    // This is where we would start to pull int he match
    // expression as a control flow
}
