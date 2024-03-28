/////////////////////////////////////////////////////
// User
//
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User{
    User {
        username: username,
        email: email,
        sign_in_count: 1,
        active: true,
    }
}

fn print_user(user: User){
    println!("{}",user.username);
    println!("{}",user.email);
    println!("{}",user.sign_in_count);
    println!("{}",user.active);
}

/////////////////////////////////////////////////////
// Rectangle
//
struct Rectangle {
    width: u32,
    height: u32
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area10(&self) -> u32 {
        self.width * self.height * 10
    }
}
/////////////////////////////////////////////////////
//
//
fn main() {
    println!("---struct User---");

    let user1 = User {
        username: String::from("MikeT"),
        email: String::from("mike@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    print_user(user1);

    let user2 = build_user("Tom".to_string(), "tom@gmail.com".to_string());
    print_user(user2);

    println!("---struct Rectangle---");
    let rect = Rectangle { width: 20, height: 10 };
    println!("area = {}", area(&rect));
    println!("area10 method way = {}", rect.area10());
}
