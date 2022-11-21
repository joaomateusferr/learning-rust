use std::io;

fn main() {

    let mut buffer = String::new();

    println!("Insert a number");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number : i8 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    //The % operation can only be used with integer values.
    //Furthermore, being divisible by 2 is the same as being even and not being divisible by 2 is the same as being odd.

    if number % 2 == 0 {
        println!("Is divisible by 2");
    } else {
        println!("It is not divisible by 2");
    }
    
}