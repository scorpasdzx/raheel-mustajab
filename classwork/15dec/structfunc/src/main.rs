struct Student{
    name:String,
    sub1:i32,
    sub2:i32
}
fn main() {
    let stud1 = Student{
        name: String::from("wajid"),
        sub1: 80,
        sub2:60
    };
    let stud2 = (String::from("wajid Ali"), 80, 60);
    studenttup(&stud2);
    studentstr(&stud1);
}
fn studenttup(stud:&(String, i32, i32)){
    println!(" Tuple values are {} {} {}", stud.0, stud.1, stud.2);
}

fn studentstr(student: &Student){
    println!(" Struct values are {} {} {}", student.name, student.sub1, student.sub2);
}