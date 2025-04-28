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

fn main() {
    let mut user1 = build_user(String::from("leo@gmail.com"), String::from("leo"));

    user1.email = String::from("muyloco@gmail.com");

    let user2 = User {
        email: String::from("leo2@gmail.com"),
        ..user1
    };

    println!("New user email: {}", user2.email);
    println!("User email: {}", user1.email);
}
