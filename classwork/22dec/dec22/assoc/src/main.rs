#[derive(Debug)]
struct Food {
    name: String,
    price: u16,
    serving: u8
}
impl Food { //associated function is in which self is not passed as argument and called Food::function name
    fn newinstance(name:String, price:u16, serving:u8) ->Food{ //return food.clone() if arguments are passed as reference
        let food = Food {
            name,
            price,
            serving
        };
        food
    }
}
fn main() {
    let food ="biryani".to_string();
    let food1 ="pulao".to_string();
    let price = 255;
    let serving = 10;
    println!("{:#?}",structmaker(food, price, serving)); //otherwise food cannot be be snet below
    println!("{:#?}", Food::newinstance(food1, price, serving));

}
fn structmaker(name:String, price:u16, serving:u8)->Food{
    let food = Food{
        name,
        price,
        serving
    };
    food
} 
