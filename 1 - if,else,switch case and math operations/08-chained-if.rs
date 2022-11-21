use std::io;

fn main() {

    let mut buffer = String::new();
    
    println!("Enter salary amount!");
    
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let salary : f32 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    let readjustment : f32;
    
    if salary < 500.0 {
        readjustment = salary*0.15;
    } else {
        if salary < 1000.0 {
            readjustment = salary*0.1;
        } else {
            readjustment = salary *0.05;
        }
    }

    println!("The salary readjustment is ${:.2}",readjustment);
    println!("The new salary is ${:.2}",salary+readjustment);

}