use std::io;

fn main() {

    let mut buffer = String::new();
    
    println!("Enter side 1 of the triangle:");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let side_1 : f32 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    println!("Enter side 2 of the triangle:");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let side_2 : f32 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();

    println!("Enter side 3 of the triangle:");
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    let side_3 : f32 = buffer.trim().parse().expect("Please type a number!");
    buffer.clear();
    
    if side_1 < side_2 + side_3 && side_2 < side_1 + side_3 && side_3 < side_1 + side_2 {

        if side_1 == side_2 && side_2 == side_3 {
            println!("Equilateral triangle");
        } else {
    
            if side_1 != side_2 && side_2 != side_3 {
                println!("Scalene triangle");
            } else {
                println!("Isosceles triangle");
            }
        }
    } else {
        println!("It's not a triangle");
    }

}