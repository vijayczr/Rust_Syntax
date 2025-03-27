// use std::vec;

use std::collections::HashMap;

fn main() { 

    // initialising a vector

    // let mut v:Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);
    
    // println!("Hello, world! {}", v[0]);

    // let third = &v[2].clone();
    // // v.push(4);
    // println!("The third element is {}", third);

    // println!("Enter a index to find:");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // // let ind: usize = input.trim().parse().expect("Please enter a valid number");

    // let ind: usize = input.trim().parse().expect("Please enter a valid number");
    // let fourth = v.get(ind);
    // match fourth {
    //     Some(fourth) => println!("The fourth element is {}", fourth),
    //     None => println!("There is no fourth element"),
    // }

    // initialising a vector

    // let mut v = vec![1, 2, 3, 4, 5];

    // for i in &v {
    //     println!("The element is {}", i);
    // }

    // for i in &mut v {
    //     *i += 50;
    //     println!("The element is {}", i);
    // }



    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);


    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");


    // Vectors, strings, and hash maps will provide a large amount of functionality necessary in programs when you need to store, access, and modify data. Here are some exercises you should now be equipped to solve:

    // Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list.
    // Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!
    // Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    // The standard library API documentation describes methods that vectors, strings, and hash maps have that will be helpful for these exercises!


}
