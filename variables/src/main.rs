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

    let c = 'z';
    println!("c is {c}");

    let heart_eyed_cat = '😻';
    println!("heart eyed cat is {heart_eyed_cat}");
}
