fn main() {
    let user1  = User {
        active: true,
        username: String::from("tripg"),
        email: String::from("tripg@mail.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.sign_in_count);
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}