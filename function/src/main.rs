fn main() {
    another_function();

    func_with_param(5);


    let y = {
        let x = 3;
        x + 1
    };

    println!("y is {y}");

    let x = five();
    println!("x is {x}");
}


fn another_function() {
    println!("Another function.");
}

fn func_with_param(x: i32) {
    println!("x is {x}");
}

fn five() -> i32 {
    5
}