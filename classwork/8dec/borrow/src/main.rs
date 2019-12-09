//borrow is pass by reference
/*fn main(){
    let team1 = String::from("Islamabad United");
    println!(" data is{}", team1);
    println!("pointer is {:p}", &team1);
    //let team2 = team1;
    //let team3 = &team2;
    let team2= &team1;
    println!("data is {}", team2);
    println!(" pointer is {:p}", &team2);
    println!(" The original data {} is still available", team1); //here as team1 is only passed by refernece therefore no move happened here for team2 from team1
    let len = t20(&team1);  //argument should match function signature means we must pass reference
    println!(" Team1 length is {}", len);
} //drop only calls for team1 not team3 because team3 is borrowing from team1 not copying/movinh
fn t20(team5:&String)->usize{
    let length = team5.len();
    length
}*/
/*mutable reference
fn main(){
    let mut football_player = String::from("Christiano");
    println!("{}", football_player);
    football_player.push_str("Ronaldo");
    println!("{}", football_player);
    let football_player1 = &mut football_player; //this will change both variables at 25 and 21 
    football_player1.push_str("Ronaldo");  //until this variable is used you cannot access football_player
    println!(" the copy {}", football_player1);
    let mut team2 = String::from("Lyari");
    println!("Only team name {}", team2);
    football(&mut team2);
    println!("the complete team name is {}", team2);
} //two mutable reference cannot be given at one time until we read or write first mutable reference we can use the second muable reference
fn football(p2:&mut String){
    p2.push_str(" Karachi");
}*/
//Dangling reference
fn main(){
    println!("we got from bacth3 {}", batch3());
}
fn batch3()->&String{   //reference could not be returned because data will not be avaialbale in main because of drop function
        let data = String::from("IOT");
        &data
}// chapter4.3 is not included which is based on slcing