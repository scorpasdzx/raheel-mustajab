fn main() {
    let num = 33;
    if num+7 == 40{
        println!("Daud From Dera Ghazi Khan");
    }
    else if num+5 == 38{
        println!("Wajid from Pindi"); //Although condition is true, but this scope wouldn't run because uper scope is true before it
    }
}
