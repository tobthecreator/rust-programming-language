/*
    Structs structure related data

    A struct is like an object's data attributes

    Structs and enums are the building blocks of types
*/

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    // if we make userA mutable, the whole record
    // is mutable
    let user_a = User {
        email: String::from("tyler@tyler.com"),
        username: String::from("tyler"),
        active: true,
        sign_in_count: 1,
    };

    // dot notation access and updates
    println!("{}", user_a.email);

    let user_b = build_user(String::from("tyler2@tyler.com"), String::from("tyler2"));
    println!("{:?}", user_b);

    let user_c: User = {
        let temp = build_user(String::from("tyler3@tyler.com"), String::from("tyler3"));
        temp
        // ..user_b
    };
    println!("{:?}", user_c);

    // .. operator fills in fields not explictly set
    let user_d = User {
        email: String::from("tyler4@tyler.com"),
        username: String::from("tyler4"),
        ..user_c
    };
    println!("{:?}", user_d);

    let black = Color(0, 0, 0);
    println!("{:?}", black);

    let origin = Point(0, 0, 0);
    println!("{:?}", origin);
}
