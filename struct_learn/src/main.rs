use std::io;
// derive(Debug)
#[derive(Debug)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

struct Area {
    width: Option<u32>,
    height: u32,
}

impl Area {
    fn area(&self) -> u32 {
        self.width.unwrap_or(0) * self.height
    }
}


fn main() {
    // let uset = User {
    //     email: String::from("vjvijay130@gmail.com"),
    //     username: String::from("vijay"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // println!("Username: {}", uset.username);
    // println!("Email: {}", uset.email);
    // println!("Sign-in count: {}", uset.sign_in_count);
    // println!("Active: {}", uset.active);

    // let user2 = User {
    //     email: String::from("anothher@exaple.com"),
    //     ..uset                                                            // value got borrowed; so, it can't be used again
    // };
    // println!("Username: {}", uset.username);       


    println!("Hello, world!");

    let mut len = String::new();
    let mut wide = String::new();

    println!("Enter the length of the rectangle: ");
    io::stdin().read_line(&mut len).expect("Failed to read line");
    let len: u32 = len.trim().parse().expect("Please type a number!");
    println!("Enter the width of the rectangle: ");
    io::stdin().read_line(&mut wide).expect("Failed to read line");
    let wide: u32 = wide.trim().parse().expect("Please type a number!");


    let rect1 = Area {
        width: Some(wide),
        height: len,
    };

    println!("The rectangle is {:?}", rect1);
    dbg!(&rect1);

    area(&rect1);
    println!("The area of the rectangle is {}", area(&rect1));
    println!("The area of the rectangle is {}", rect1.area());                                 // method call

    println!("thhe length is {}", len);

}

fn area(rect : &Area) -> u32 {
    rect.width.unwrap_or(0) * rect.height
}
