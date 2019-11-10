fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    println!("The home: {:?}", home);
    let loopback = IpAddrKind::V6(String::from("::1"));
    println!("The loopback: {:?}", loopback);
    let ip_v4_addres = IpAddr::V4(127, 0, 0, 1);
    println!("The ipV4Addres: {:?}", ip_v4_addres);
    let ip_v6_addres = IpAddr::V6(String::from("::1"));
    println!("The ipV6Addres: {:?}", ip_v6_addres);
}

#[derive(Debug)]
// enum with attached data
enum IpAddrKind {
    V4(String),
    V6(String),
}

// enum with data of different types and associated data
// advantage over struct
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}