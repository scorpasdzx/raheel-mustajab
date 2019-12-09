fn main() {
    let mut age = 22;                    //datatype could not change like we shadowing
    println!("value of age is: {}", age);
    println!(" pointer of age is: {:p}",&age);
    age = 33;
    println!("update value of age is: {}",age);
    println!("new pinter of age is: {:p}", &age);
    let salary = 45_000;                       
    println!("salary is:{}", salary);
    println!("pointer is :{:p}", &salary);
    let salary = salary+3_000;                  //shadowing start with let keyword
    println!("new salary is:{}", salary);
    println!("new pointer is :{:p}", &salary);
    let salary = "Three Thousands";             //shadowing datat type change
    println!("new salary is:{}", salary);
    println!("new pointer is :{:p}", &salary);
    let _age:u8 = 22; //will take 1byte memory address
    let _age:u16 = 22; //will take 2bytes in memory address
    let _age:u32 = 22; //will take 4bytes memory address
    println!("age value is:{}", age); //by default integer is 32 byte unless explicitlly mentioned
    //for float :f32 :f64 by default 64 is set unless explicitly mentioned
    // constant is declared as const NAMEINCAPITAL f : 32 (it doesn't take emory address)
    //constant is replaced by its value at compile time
    //constant can be declared globally: outside fb main(), but simple variable 
    //cannot declare outside fn main()
}