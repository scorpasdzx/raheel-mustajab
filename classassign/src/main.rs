use std::io;

fn main(){
println!("Enter any number");
let mut inputdata = String::new();
io::stdin().read_line(&mut inputdata).expect("failed to read line");
let int_data = inputdata.trim_end().parse::<i32>().unwrap();
/*if int_data ==1 {
    println!("One");
}
else if int_data==2 {
    println!("two");
}
else if int_data==3 {
    println!("three");
}
else {
    println!("Not in list");
}
}*/
match int_data {
    1=> println!("one"),
    2=> println!("two"),
    3=>println!("Three"),
    _=>println!("Not in list"),
}
}