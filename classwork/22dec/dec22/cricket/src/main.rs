#[derive(Debug)] //for clone write Clone beside Debug
struct Teams {
    country:String,
    score:u16
}
impl Teams {   //method is always defined inside impl and parameter is self
    fn high(&self,other:&Teams)->u16{  //function if not method then can be created anyware outside
        if self.score > other.score {
            self.score
        }
        else {
            other.score
        }
    } 
    fn higher(self,other:Teams)->Teams {
        if self.score > other.score {
            self
        }
        else {
            other
        }
    }
}
fn main(){
    let team1 = Teams {
        country: "Pakistan".to_string(),
        score: 435
    };
    let team2 = Teams {
        country: "Srilanka".to_string(),
        score: 271
    };
    println!("{}", &team1.high(&team2));
    println!("{:#?}",team1.higher(team2));  //since the impl func is not reurning we print inside impl
}