fn main() {
   println!("V4: {:?}", IpAddrKind::V4);
   
   let home = IpAddr {
       kind: IpAddrKind::V4,
       address: String::from("127.0.0.1"),
   };
   println!("The home: {:?}", home);

   let loopback = IpAddr {
       kind: IpAddrKind::V6,
       address: String::from("::1"),
   };
   println!("The loopback: {:?}", loopback);
}

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
