use std::io;

fn main() {

    let mut buffer = String::new();
    
    println!("Enter 3 numbers");
    
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number1 : i16 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number2 : i16 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let number3 : i16 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    let sum: i32 = (number1 + number2 + number3).into();

    if sum >= 100 {
        println!("The sum is greater than or equal to 100");
    } else {
        println!("The sum is less than 100");
    }

}