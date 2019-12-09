fn main() {
    let mut i = 1;
    while i <= 10 {
        if i == 3 || i == 7 || i == 10 {
            println!("Special security check");  //the single if statement is more efficient then three seperate iff statement because it will make our program run faster
        }
    i = i + 1;
    }
}