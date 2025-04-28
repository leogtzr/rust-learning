struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user = User {
        active: false,
        username: String::from("leogtzr"),
        email: String::from("leo@gmail.com"),
        sign_in_count: 34,
    };

    println!("active: {}, email: {}", user.active, user.email);

    user.email = String::from("leo@hotmail.com");

    println!("active: {}, email: {}", user.active, user.email);

    let user2 = build_user(String::from("perla@perla.com"), String::from("perla"));
    println!("Perla is: {}, {}", user2.email, user2.username);
}
