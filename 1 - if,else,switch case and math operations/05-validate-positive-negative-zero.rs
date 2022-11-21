use std::io;

fn main() {

    let mut buffer = String::new();

    println!("Insert a number");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number : i8 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    if number > 0 {
        println!("It's positive");
    } else {
        if number < 0 {
            println!("It's negative");
        } else{
            println!("It's zero");
        }
    }
    
}