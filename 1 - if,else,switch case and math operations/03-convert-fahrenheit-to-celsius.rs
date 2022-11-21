use std::io;

fn main() {

    let mut buffer = String::new();

    println!("Enter the temperature in fahrenheit");
    
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let fahrenheit : f32 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    let celsius : f32 = (fahrenheit-32.0)*5.0/9.0;

    println!("The temperature in celsius is {:.2}", celsius);
    
}