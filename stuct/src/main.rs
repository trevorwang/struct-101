fn main() {
    let user1 = build_user(String::from("hello@qq.com"), String::from("trevro"));

    println!("user's email is {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    println!("user2's email is {}", user2.email);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
    println!("user3's email is : {}", user3.email);

    let _color = Color(1, 3, 12);

    // rectangle
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!("rectable's area is {}", area(&rect));
    println!("rect is {:?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn _build_user(email: String, username: String) -> User {
    User {
        email: String::from(email),
        username: String::from(username),
        active: true,
        sign_in_count: 1,
    }
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(u32, u32, u32);
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    fn squre(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
