use std::io;
fn determiner(num:&str){
if num > "0" {
    println!("YOur entered number is positive");
}
else if num < "0" {
    println!("Your entered number is negative");
}
else if num == "0" {
    println!("Your entered number is zero");
}
}
fn main() {
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read input");
    determiner(&num);
}
