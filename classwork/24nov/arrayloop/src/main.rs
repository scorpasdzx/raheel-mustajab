fn main(){
    let age: [i32;5] = [10,20,30,40,50];
    for value in age.iter() {
        println!("the value is {}", value.pow(2));  //for f32 use powi(2)
    }
}