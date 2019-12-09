fn main() {
    let get = giveback();
    println!("we get {}", get);    
    let batch3= String::from("iot"); //:: for associated function, . for methods
    let assignment = receiver(batch3);
    println!("we got is assignment {:?} ", assignment);
    println!("The length of {} is {}", assignment.0, assignment.1);
    let (zero,one) = receiver(get);
    println!("the length of {} is {}", zero,one);
}
fn giveback()->String{
    let name = String::from("batch3");
    name //no drop will be called for name after } because ownership gets tranfer
}
fn receiver(morning:String)->(String, usize){
    println!("We are in receiver function");
    println!("{}", morning);
    let length = morning.len();
    (morning, length) //no drop function will be called for morning
}
//testing comment