/*fn main(){
    printer(500);
}*/

fn printer(copy:u32){ //function signature
    println!("print {} copies", copy);
}

fn main(){
    let order = 500;
    printer(order);
}