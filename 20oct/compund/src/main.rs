fn main() {
 let mut data = ("PIAIC", 3000, 67.8);
 let (mut _org, mut fee, mut _percent) = data;
 println!("The fee is: {}", fee);
 let org1 = data.0;
 println!(" The organization is: {}", org1);
 println!("Complete tuple: {:?}", data);
 data.1 = 2000;
 fee = data.1;
 println!("updated fee is:{}", data.1);
 println!("New complete tuple: {:?}", data);
 println!(" new fee is: {}", fee);
 //let a = [1,2,3,4,5];
 //let first = a[0];
 //let second = a[1];
 //println!("first is: {} and second is {} ", first, second);
 //println!("complete array {:?}", a);
 //println!(" Fourth index is {}", a[4]); //println! prints in new line while print! continues with same line
//let a:[i32;5];
//let a = [3;5];
//println!(" Fourth index is {}", a[8]); //this will give compile time error
//let temp = 8;
//println!(" Fourth index is {}", a[temp]); 
}