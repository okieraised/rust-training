fn main() {
    let user1  = User {
        active: true,
        username: String::from("tripg"),
        email: String::from("tripg@mail.com"),
        sign_in_count: 1,
    };

    println!("{}", user1.sign_in_count);

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("tripg2@mail.com"),
        sign_in_count: 1,
    };

    let user3 = User {
        email: String::from("tripg3@mail.com"),
        username: String::from("haha"),
        ..user1
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}