fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    
    let loopback = IpAddr::V6(String::from("::1"));

    let five = Some(5);
    let six = plus_one(five);
    println!("{:#?}",six);
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddr {
    V4(String),
    V6(String),
}

// enum Option<T> {
//     Some(T),
//     None,
// }

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
