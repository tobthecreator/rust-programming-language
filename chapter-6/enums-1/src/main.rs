#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Can bundle with data in a struct
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Instead we can attach data directly to the enum
#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    // V6(String),
}

// Inside a struct, all values for address would have to be the same
// In an enum, they can be different
#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
    // V6(String),
}

struct Ipv4Addr {}

struct Ipv6Addr {}

enum IpAddr4 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    println!("{:?}", four);
    println!("{:?}", six);

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", home);

    let home2 = IpAddr2::V4(String::from("test"));
    println!("{:?}", home2);

    let home3 = IpAddr3::V4(1, 2, 3, 4);
    println!("{:?}", home3);
}

fn route(ip_kind: IpAddrKind) {}
