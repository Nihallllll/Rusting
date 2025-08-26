mod shadowing;
mod pattern_matching;
use pattern_matching::{Animal, pattern_matching};
mod refrences;
mod traits;
mod Lifetimes;
fn main() {
    shadowing::shadowing();
    let jungle_king = Animal::Tiger(String::from("Sher Khan"));
    
    // Call the function again
     pattern_matching(jungle_king); 
     refrences::references();
     traits::traits();
     Lifetimes::checker();

     let x ="nijhal";
     let y= x ;
     println!("y : {}",y);
}
