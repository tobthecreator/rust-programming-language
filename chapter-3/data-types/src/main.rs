fn main() {
    /*
        Scalar types
        integers, floating point, booleans, characters
        i/u8
        i/u16
        i/u32
        i/u64
        i/u128

        true
        false

        char - strings with single quotes, unicode
    */

    /*
        Compound types
        Tuple
    */
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("{:?}", tup); // (500, 6.4, 1)
    println!("{}, {}, {}", x, y, z); // 500, 6.4, 1
    println!("{}, {}, {}", tup.0, tup.1, tup.2); // 500, 6.4, 1

    /*
        Compound types
        Array
    */

    let array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let months: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:?}", months);

    let array2 = [0; 5]; // [0,0,0,0,0]
    println!("{:?}", array2);

    /*
        Compound Types
        Functions
    */
    func2();
    func3(999);
    func4(998, 997);
    func5();
    func6();
    func7();
    func8();
}

// Rust does not care about order
fn func2() {
    println!("func2!");
}

// Parameters
fn func3(x: i32) {
    println!("you passed in {}!", x);
}

// Parameters 2
fn func4(x: i32, y: i32) {
    println!("you passed in {}, {}!", x, y);
}

// Function blocks
fn func5() {
    let a = 5;
    let b = {
        let a = 3; // with a semicolon this is a statement
        a + 1 // the lack of semi colon makes it an expression that returns a value
    };

    println!("The value of a is: {}, and b is: {:?}", a, b);
}

// Return values
fn num5() -> i32 {
    5 // an expression
}

fn func6() {
    let x = num5();

    println!("the value of x is: {}", x);
}

// If a semicolon were used below, it'd be a statement
// and statements do not return values.println!
// so a return would be needed
fn plus_one(x: i32) -> i32 {
    x + 1
}

fn func7() {
    let x: i32 = 3003;

    let x = plus_one(x);

    println!("the value of x is: {}", x);
}

fn plus_two(x: i32) -> i32 {
    let x = x + 2;

    return x;
}

fn func8() {
    let x: i32 = 3003;

    let x = plus_two(x);

    println!("the value of x is: {}", x);
}
