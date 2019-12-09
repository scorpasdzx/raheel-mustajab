// const MAX_PRICE:f32 = 5000; it will generate error remove comments to see
fn main() {
    let age:u8 = 44; //by default i32
    println!("value of age is:{}", age);
    let temp = 16.23; //by default f64
    println!("temprature is:{}", temp);
    let salary:usize=45_000;  //for 64bit system it is 64bit 
    println!("salary is:{}", salary);
    let rent = 36_000u16;  //another way to u16: either rent:u16 or defined in this example
    println!("rent is : {}",rent);
    let salary ="Three Thousands";
    println!("new salay is:{}", salary.len());
    println!("new salary is:{}", salary);
    //shadowing create new address each time while mut remain at same memory location
    let t = true;
    let f:bool = false; //with explicit type annotation
    let data2 = 'z';
    let data1 = 'Z';
}