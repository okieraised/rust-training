use std::io;

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

    let spaces = "    ";

    let spaces = spaces.len();

    println!("len spaces is {spaces}");

    // Numeric
    let sum = 5 + 10;
    println!("sum is {sum}");

    let difference = 95.5 - 4.3;
    println!("difference is {difference}");

    let product = 4 * 30;
    println!("product is {product}");

    let quotient = 56.7 / 32.2;
    println!("quotient is {quotient}");

    let truncated = -5 / 3;
    println!("truncated is {truncated}");

    let remainder = 43 % 5;
    println!("remainder is {remainder}");

    // Char type
    let c = 'z';
    println!("c is {c}");

    let heart_eyed_cat = '😻';
    println!("heart eyed cat is {heart_eyed_cat}");

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("tuple: x is {x}, y is {y}, z is {z}");

    let first = tup.0;

    let second = tup.1;

    let third = tup.2;

    println!("first is {first}, second is {second}, third is {third}");

    // Array
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("failed to read line");

    let index: usize = index.trim().parse().expect("Index entered was not a number");

    let element = a[index];

    println!("value at index {index}: {element}");
}
