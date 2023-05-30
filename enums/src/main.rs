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

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
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
    println!("coin value is {}", val);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }


    let config_max = Some(6u8);
    match config_max {
        Some(max) => println!("the max is configured to be {}", max),
        _ => (),
    }

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("the max is configured to be {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Oklahoma);
    if let Coin::Quarter(state) = coin {
        println!("state is {:?}", state);
    } else {
        count += 1;
    }

}

fn route(ip_kind: IpAddrKind) {}

fn add_fancy_hat() {
    println!("haha")
}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("{}", num_spaces)
}