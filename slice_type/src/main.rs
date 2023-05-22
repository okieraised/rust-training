fn main() {
    let mut s = String::from("Hello, world!");

    let word = first_word(&s);

    println!("first word is '{}'", word);

    s.clear();

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello is '{}', world is '{}'", hello, world)
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();


    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}