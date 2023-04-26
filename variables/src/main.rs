fn main() {
    let mut x = 5;
    println!("x is {x}");

    x = 6;
    println!("x is {x}");

    const THREE_HOUR_IN_SECONDS: u32 = 60 * 60 * 3;


    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("inner x is {x}");
    }
    println!("outer x is {x}");
}
