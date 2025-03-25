

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAdd {
//     kind: IpAddrKind,
//     address: Option<String>,
// }

// use std::default;

enum colors {
    Red,
    Yellow,
    Green,
    Blue,
    
}

fn print_color(colors: colors) {
    // let color = colors::Red;
    match colors {
        colors::Red => println!("Red"),
        colors::Green => println!("Green"),
        colors::Blue => println!("Blue"),
        colors::Yellow => println!("Yellow"),
    }
}

impl colors {
    fn makes_green(&self) -> bool {
        match self {
            colors::Blue => true,
            colors::Yellow => true,
            _ => false,
        }
    }

    fn is_Green(&self) -> bool {
        if let colors::Green = self {
            true
        } else {
            false
        }
    }
}

fn main() {
    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // let localhost = IpAdd{
    //     kind: IpAddrKind::V4,
    //     address: Some(String::from("126.0.0.1"))
    // };
    // let val: Option<i32> = Some(5);

    let x = 5;
    let y: Option<i8>  = None;

    let sum = x + y.unwrap_or(0);

    println!("Hello, world! {}", sum);
    print_color(colors::Red);
}
