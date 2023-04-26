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


}

fn takes_ownership(some_string: String) {
    println!("some string: {some_string}");
}


fn make_copy(some_integer: i32) {
    println!("{some_integer}");
}