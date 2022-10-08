fn main() {
    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));
}

enum IpAddr {
    V4(String),
    V6(String),
}
