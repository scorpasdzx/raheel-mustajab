#[derive(Debug)]
struct Student{      //structure, instructure definition it is key and datatype
    name:String,    //key datatype   
    subject1:i32,         //struct is userdefined datatype, and its variable declaration is called creating instance
    subject2:i32      //struct is always stored on heap despite its field for being primitive
}

/*struct Food {     //it is recommended that struct name should be camel case
    name:String,
    price:u32
}*/

fn main() {
    let name = String::from("wajid ");
    let sub1= 80;
    let sub2 = 60;
    let st1 = (String::from("ALI"),80,80); //tuple making
    println!("{:?}", st1);
    println!("{}",st1.0);

    let stud1 = Student {    //structure instantiation
        name:String::from("wajid"),  //after instantiation it is called key value pair
        subject1: 80,
        subject2: 90
    };
    println!("name: {}", stud1.name);
    println!(" subject1 marks: {}", stud1.subject1);
    println!(" subject2 marks: {}", stud1.subject2);
    println!(" name: {:?}", stud1); //doesnot implement formatting :?, either call by fields.
    //to print structure with like :? for tupple add #[derive(Debug)] just before each struct 

    let mut stud2 = Student{
        name:String::from("Rabi darling"),
        subject2: 85,
        subject1: 100
    };
    //struct can be put in different order, and data should be put forr all keys
    //to make struct mutable its instance just need to be mutable and it is done for the whole struct instead of individual keys
    let stud3 = stud2;
    println!("{:?}", stud3);
    //concept of ownership applies to structures
    let stud4 = Student {
        name: String::from("Sania Amjad"),
        subject1: stud3.subject1,
        subject2: stud3.subject2
    };
    let stud5 = Student{
        name:String::from("Laiba"),
        ..stud4  //this specifies that the rest of the struct field to be taken by stud4
    };
}