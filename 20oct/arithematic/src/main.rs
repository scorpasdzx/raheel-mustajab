fn main() {
    let _value1 = 22;
    let _value2 = 2;
    let _plus= _value1 + _value2;
    let _minus = 22-2;
    let _div = _value1/_value2;
    let _mul = _value1*_value2;
    let _mod=_value1%_value2;
    println!("plus {} + {} = {}", _value1,_value2,_plus);
    println!("min {} - {}= {}", _value1,_value2,_minus);
    println!("div {} / {} = {}", _value1,_value2,_div);
    println!("mul {} * {} = {}", _value1,_value2,_mul);
    println!("mod {} / {} = {}", _value1,_value2,_mod);
    let _name = "PIAIC Batch 3";
    println!("String Literal: {}", _name);
    let _grade = 'A';
    let _enrol = true;
    println!(" Grade: {}  Enrollement status: {}", _grade, _enrol);
    let _emoji = '\u{1F633}';
    println!("{}", _emoji);
}
