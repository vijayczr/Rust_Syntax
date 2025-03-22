fn main() {
    // let x =5;
    // let x = x+3;
    // {
    //     let x = x*2;
    //     println!("The value of x in the inner scope is: {}", x);
    //     let y =  "    ";
    //     let val = y.len();
    //     println!("The length of y is: {}", val);
    // }
    // let y = 3.3;
    // // let f = true;
    // // let c = 'z';
    // let emoji = '😭';
    // // let tup = (500, 6.4, 1)
    // println!("The value of emoji is: {}", emoji);
    // println!("The value of y is: {}", y);
    // println!("The value of x is: {}", x);


    // let list = [1, 2, 3, 4, 5];

    // println!("Please enter your array index");
    // let mut index = String::new();
    // std::io::stdin().read_line(&mut index).expect("Failed to read line");
    // let index: usize = index.trim().parse().expect("Please type a number");
    // let element = list[index];
    // println!("The value of the element is: {}", element);   

    // println!("Now we will do an addition function please enter a number");
    // println!("Enter ffirst number : ");
    // let mut num1 = String::new();
    // std::io::stdin().read_line(&mut num1).expect("Failed to read line");
    // let num1: u32 = num1.trim().parse().expect("Please type a number");
    // println!("Enter second number : ");
    // let mut num2 = String::new();
    // std::io::stdin().read_line(&mut num2).expect("Failed to read line");
    // let num2: u32 = num2.trim().parse().expect("Please type a number");

    // let res = another_function(num1,num2);
    // println!("The result of the addition is: {}", res);

    // loop {
    //     println!("again!");
    // }
 
    // let mut counter = 0;
    // let result = loop{
    //     counter +=1;
    //     if counter ==10 {
    //         break counter*2;
    //     }
    // };
    // println!("The result of the loop is: {}", result);

    // let mut val = list.len()-1;
    // while val != 0{
    //     println!("The value of val is: {}", list[val]);
    //     val -=1;
    // }


    // let t1 = String::from("Hello");                                           //ownersip and reference
    //                                                                                   //
    // let t2 = takes_ownership(&t1);                                              //                                              
    // println!("The value of t2 is: {}{}", t2,t1);                           //


    let mut s = String::from("hello world");
    change(&mut s);
    println!("The value of s is: {}", s);


}

fn change(s: &mut String){
    s.push_str(" and good morning");
}

// fn takes_ownership(s: &String) -> &String {
//     println!("The value of s is: {}", s);
//     s
// }

// fn another_function(x: u32 ,y: u32)  -> u32 {
//     return x+y;
// }


