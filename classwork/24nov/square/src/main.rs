fn main() {
    let number:u32 = 5;
    let receive = square(number);
    println!("square of {} is {}", number, receive.1);
}

fn square(val:u32) ->(u32, u32){
    (val, val.pow(2))
}