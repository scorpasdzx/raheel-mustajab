use std::io;
fn main() {
  let mut input1 = String::new();
  println!("Enter your data");
  io::stdin().read_line(&mut input1).expect("Failed to read input");
  println!("You enetered: {}", input1);
  println!("You entered: {:?}", input1);
  println!("You entered: {:?}", input1.trim());
  let int_input:u32 = input1.trim().parse().expect("Please type a number"); //parse do conversion and :u32 do casting
  println!("You enetered: {}", int_input); 
// in panic there is some compiling and the program run into error at some point, but compiler error don't run at all
}

