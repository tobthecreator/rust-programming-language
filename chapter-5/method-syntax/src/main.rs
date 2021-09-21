#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

// implementation block
// mutliple impl blocks can be created for the same struct
impl Rectangle {
    // we're using &self here so this function
    // doesn't take ownership
    //
    // to take ownership you'd use just 'self'
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, cmp_rect: &Rectangle) -> bool {
        self.area() >= cmp_rect.area()
    }

    // Associated function
    // key feature is that it lacks a reference to self
    // accessible with '::' instead of '.'
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("{}", rect1.area());

    let rect2 = Rectangle {
        width: 3,
        height: 5,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 30,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect1? {}", rect2.can_hold(&rect1));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(5);
    println!("Square: {:?}", square);
}
