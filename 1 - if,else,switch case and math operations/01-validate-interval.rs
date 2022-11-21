use std::io;

fn main() {

    let mut buffer = String::new();

    println!("Insert a number");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number : i8 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    if number >= -1 && number <= 1 {
        println!("It's in the range between -1 and 1");
    } else {
        println!("It's not in the range between -1 and 1");
    }
    
}