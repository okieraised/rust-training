fn main() {
    let number = 5;

    // if - else
    if number < 5 {
        println!("false")
    } else {
        println!("true")
    }

    // Multiple if - else
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }


    let mut counter = 0;

    let result = loop {
        
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("result is {result}");

    // Using loop identifier
    let mut count = 0;
    'counting_up:  loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1
        }

        count += 1
    }

    println!("End count = {count}");


    // Using while
    let mut number = 3;

    while number != 0 {
        println!("number is {number}");
        number -= 1;
    };

    println!("lift off");

    // Looping through collection
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is {}", a[index]);
        index += 1;
    }

    // Concise for loop
    for elem in a {
        println!("the value is {elem}");
    }

    // For loop in reverse
    for number in (1..4).rev() {
        println!("reverse is {number}");
    }

}
