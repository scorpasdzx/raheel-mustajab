use std::io;
fn main() {
    let mut temp = String::new();
    println!("Please enter your height in feets");  //usually we enter height in feets in our daily use
    io::stdin().read_line(&mut temp).expect(" Failed to read input");
    let mut height:f32 = temp.trim().parse().expect("Please enter height in number");
    height = height / 3.2808; //to convert height in feet to meters
    let mut temp2 = String::new();;
    println!("Now enter your weight in kilograms");
    io::stdin().read_line(&mut temp2).expect("Failed to read input");
    let weight:f32 = temp2.trim().parse().expect(" Please enter height in number form"); //otherwise compiler will give error because height is in float format
    let bmi = weight / height;
    println!(" bmi: {}", bmi);
    if bmi < 18.5 {
        println!("You are underweight");
    }
    else if bmi > 18.5 && bmi < 25.0{
        println!("Your weight is normal");
    }
    else if bmi > 25.0 {
        println!("You are overweight");
    }
}