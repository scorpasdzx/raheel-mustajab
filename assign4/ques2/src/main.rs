fn main() {
    let num : u32 = 1;
    let argument = (num, 0.1f32, true);  //why we cant declare a variable as tuple element and input it using user input mehtod
    ques2(argument);
}

fn ques2(arg: (u32,f32,bool)) {
    println!("The integer value is: {:?}", arg.0);
    println!("The float value is: {:?}", arg.1);
    println!("The boolean value is: {:?}", arg.2);
}