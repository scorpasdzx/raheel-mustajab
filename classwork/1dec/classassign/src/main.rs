fn main() {
    let actual_price:f32 = 2000.0;
    let discount_price:f32 = 1900.0;
    let customer_paid:f32 = 5000.0;
    let result =  disc(actual_price, discount_price, customer_paid);
    if result.1 < 0.1{
        println!(" You have availed Azadi sale");
        println!(" Your remaining balance is : {}", result.2);
    }
    else if result.1 >=0.1 && result.1 <0.2 {
        println!(" You have availed Eid sale");
        println!(" Your remaining balance is : {}", result.2);
    }
    else if result.1 >0.2 {
        println!(" You have availed Mojaan sale");
        println!(" Your remaining balance is : {}", result.2);
    }
}
fn disc( act:f32, disc:f32, cust:f32) -> (f32,f32,f32) {
let perc:f32 = 1.0-(disc/act);
let balan:f32 = cust-disc;
(act,perc,balan)
} 