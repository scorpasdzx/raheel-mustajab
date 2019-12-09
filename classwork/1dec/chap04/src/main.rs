fn main() {
    {
        let age:u8 = 55;      //variable is pushed on stack
    println!("Age os : {}",age);
    println!("Pointer of age is : {:p}", &age);
    }
    //println!("Age is : {}", age);  //age is out of scope so cannot be acessed here
    {
        let student_name = String::from("Wajid Ali"); //String always store on heap
        println!("Name is :{}", student_name);
        println!(" Pointer is : {:p}", &student_name);
    }
    let data = "Wajid".to_string(); //string type
    let partner = "Wajid"; //string literal
    let mut datastring = partner.to_string(); //string type
    datastring.push_str(" Ali");
    datastring.push_str(" 1987");
    println!("data : {} datastring {}", data,datastring); //when scope overs for stack pop for heap drop
    //let s1 = String::from("wajid");
    //let s2 = s1;
    //println!("Name : {}", s1); //s1 doesnot exist because ownership is transferred to s2
    let s1 = String::from("Hello");
    let s2 = s1.clone();  //when npt clone then it is move
    println!("s1 = {}, s2 = {}", s1,s2); //use randome number generation form crates.io
    
}