fn main() {
    println!("Your order of {} books is ready", printer()); //return value can be saved in a variable
}

fn printer() ->u32 {
    println!("print 500 copies");
    return 500;   // to not use return just type 500 without semiolon
}