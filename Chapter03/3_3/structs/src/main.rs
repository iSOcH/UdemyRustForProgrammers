#[derive(Debug)]
struct User {
    is_admin: bool,
    username: String,
    password: String,
}

fn build_admin(username: String, password: String) -> User {
    User {
        is_admin: true,
        username,
        password,
    }
}

fn main() {
    let user1 = User {
        is_admin: true,
        username: String::from("Jan"),
        password: String::from("SubmarineFighter3"),
    };

    println!("{user1:?}");

    println!("{}", user1.is_admin);
    println!("{}", user1.username);
    println!("{}", user1.password);

    let mut user2 = build_admin(String::from("Peter"), String::from("P4ASSWORD"));

    println!("{user2:?}");

    println!("{}", user2.is_admin);
    println!("{}", user2.username);
    println!("{}", user2.password);

    user2.password = String::from("NewPassword");

    // # does "pretty print", similar to indented json
    println!("User2 pretty-printed: {user2:#?}");
}
