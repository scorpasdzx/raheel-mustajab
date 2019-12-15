struct Student{
    name:String,
    sub1:i32,
    sub2:i32
}
fn main() {
    let name = String::from("wajid ali");
    let sub1 = 80;
    let sub2 = 65;
    let stud7 = structmaker(name, sub1, sub2);
    println!(" structmaker {} {} {}", stud7.name, stud7.sub1, stud7.sub2);
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
fn structmaker(name:String, sub1:i32, sub2:i32) ->Student{
    /*let stud6 = Student{
        name:name,
        sub1:sub1,
        sub2:sub2
    }; or */
    let stud6 = Student{
        name,
        sub1,
        sub2
    }; 
    stud6
}// study tuple type struct    e.g struct stud(String,i32,i32);  semicolong is necessary when defining tupple type struct
//random github match struct implement for homework