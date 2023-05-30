fn main() {
    let mut s = String::from("hello");

    s.push_str(", world");

    {
        let s = String::from("hellooooo");
        println!("inside s: {s}");
    }
    println!("outside s: {s}");

    // Clone string
    let s2 = String::from("Halo");
    let s3 = s2.clone();

    println!("s2 = {s2}, s3 = {s3}");

    // Ownership and function
    let s4 = String::from("yoooooooo");

    takes_ownership(s4);

    let x = 5;
    make_copy(x);

    println!("x is {x}");

    // Return value and scope

    let s5 = String::from("sadasda");

    let s6 = take_and_give_ownership(s5);

    println!("s6 is {s6}");


}

fn takes_ownership(some_string: String) {
    println!("some string: {some_string}");
}


fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn give_ownership() -> String {

    let some_string = String::from("hihi");

    some_string
}

fn take_and_give_ownership(some_string: String) -> String {
    some_string
}