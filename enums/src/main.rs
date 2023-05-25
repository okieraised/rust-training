enum IpAddrKind {
    V4(String),
    V6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Oklahoma,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickle => 5,
        Coin::Dime => 1,
        Coin::Quarter(state) => {
            println!("state is {:?}", state);
            25
        },
    }
}

fn main() {

    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    route(four);
    route(six);

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));


    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;

    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    let val = value_in_cents(Coin::Quarter(UsState::Oklahoma));
    println!("coin value is {}", val)
    
}

fn route(ip_kind: IpAddrKind) {}