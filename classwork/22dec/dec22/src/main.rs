#[derive(Debug)]
struct Student {
score:i32,
name:String
}
impl Student {     //it is called method
    fn impll_print(&self){
        println!(" The complete details of impl is {:#?}", self);
    }
    fn score(&self) ->i32 {
        self.score
    }
}
fn main(){
    let std1 = Student {
        score: 85,
        name: String::from("wajid ali")
    };
    print(&std1);
    let mut guest= Student {
        name: "Taimoor Imtiaz".to_string(),
        score: 100
    };
    print(&mut guest);  //if & is not use then move error will occur
    guest.impll_print();
    println!("Score {}", guest.score());
}
//cannot use &mut in signature because it will give eror due to prin(&std1) on line 19
fn print(std:&Student){  //alternative is impl and self in the println will do the job
println!("The complete details {:#?}", std);  //:? simply print the complete struct, :#? will print complete struct pretty
}
