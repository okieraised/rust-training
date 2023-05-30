fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("length of {s1} is {len}");


    // Mutable reference
    let mut s2 = String::from("adsasdads");

    change(&mut s2);

    println!("s2 is {s2}");

    // Reference scope
    let mut s3 = String::from("mutable");

    let r1 = &s3;
    let r2 = &s3;

    println!("r1 is {r1}, r2 is {r2}");

    let r3 = &mut s3;
    println!("r3 is {r3}")

    // Slice reference
}

fn first_word(s: &String) -> usize {


    s.len()
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", adsasdasdada");
}