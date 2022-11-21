use std::io;

fn main() {

  let mut buffer = String::new();
    
  println!("Enter 3 numbers");
    
  io::stdin().read_line(&mut buffer).expect("Failed to read line!");
  let number1 : i8 = buffer.trim().parse().expect("Please type a number!");
  buffer.clear();

  io::stdin().read_line(&mut buffer).expect("Failed to read line!");
  let number2 : i8 = buffer.trim().parse().expect("Please type a number!");
  buffer.clear();

  io::stdin().read_line(&mut buffer).expect("Failed to read line!");
  let number3 : i8 = buffer.trim().parse().expect("Please type a number!");
  buffer.clear();

  let mut greater : i8 = number1;

  if number2 > greater {
    greater = number2;
  }

  if number3 > greater {
    greater = number3;
  }

  println!("{} is the greater", greater);

}