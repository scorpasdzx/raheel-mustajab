use std::io;
fn main() {
    let mut num = String::new();
    println!("Please enter an integer");
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let receive = square(&num);
    println!("Square of {:?} is {:?}", receive.0, receive.1);
}
fn square(num:&str) -> (u32, u32){
    let i:u32 = num.trim().parse().expect("Please type a number"); //why parsing dont work without expect function
    (i , i.pow(2))
}

