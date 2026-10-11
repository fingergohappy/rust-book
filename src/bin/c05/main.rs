struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn square() -> Self {
        Self {
            width: 10,
            height: 10,
        }
    }
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("finger"),
        email: String::from("xxx@gmail.com"),
        sign_in_count: 10,
    };

    let mut user2 = User {
        active: true,
        username: String::from("finger"),
        email: String::from("xxx@gmail.com"),
        sign_in_count: 10,
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect is {rect:?}");
    println!("rect area is {}", rect.area());
    let square = Rectangle::square();
    println!("square area is {}", square.area());
}
