fn main() {
    let mut value: u8 = 0;
    loop {
        println!("welcome # {} ", value);
        value = value + 1;
        // observe code behaviour by commenting above and below value statements
        println!(" welcome #{ }", value);
        if value == 5{
            break
        }
    }
}
