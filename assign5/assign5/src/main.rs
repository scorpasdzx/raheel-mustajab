use std::io;
fn main() {
    let mut name = String::new();
    let mut physics = String::new();
    let mut mathematics = String::new();
    io::stdin().read_line(&mut name).expect("Fail to read a line");
    io::stdin().read_line(&mut physics).expect("Failed to read a line");
    io::stdin().read_line(&mut mathematics).expect("Failed to read a line");
    let percent = percentage(name,physics,mathematics);
    if percent.1 >=70.0 {
        println!(" Congratulations {} you have passed in Physics", percent.0.trim_end());
    }
    else {
        println!("Better luck nexttime in Physics {}", percent.0.trim_end());
    }
    if percent.2 >= 70.0 {
        println!( "Congratulations {} you have passed in Mathematics", percent.0.trim_end());
    }
    else {
        println!("Better luck next time in Mathematics {}", percent.0.trim_end());
    }
}
fn percentage(name:String, sub1:String, sub2:String)-> (String,f32,f32) {
    let percent1 = ((sub1.trim_end().parse::<f32>().unwrap())/100.0)*100.0;
    let percent2 = ((sub2.trim_end().parse::<f32>().unwrap())/100.0)*100.0;
    println!("{} has got {:?}% marks in physics and {:?}% in mathematics", name.trim_end(), percent1, percent2);
    (name, percent1, percent2)
}