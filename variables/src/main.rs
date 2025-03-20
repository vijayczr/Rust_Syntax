fn main() {
    let x =5;
    let x = x+3;
    {
        let x = x*2;
        println!("The value of x in the inner scope is: {}", x);
        let y =  "    ";
        let val = y.len();
        println!("The length of y is: {}", val);
    }
    let y = 3.3;
    // let f = true;
    // let c = 'z';
    let emoji = '😭';
    // let tup = (500, 6.4, 1)
    println!("The value of emoji is: {}", emoji);
    println!("The value of y is: {}", y);
    println!("The value of x is: {}", x);
}
