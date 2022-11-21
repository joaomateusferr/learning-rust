use std::io;

fn main() {

    let mut buffer = String::new();
    
    println!("Insert a character");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let character : char = buffer.trim().parse().expect("Please type a character!");
    buffer.clear();

    if character == 's' || character == 'n' {
        println!("It's s or n");
    } else {
        println!("It's not s or n");
    }

}