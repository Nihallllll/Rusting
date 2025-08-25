mod shadowing;
mod pattern_matching;
use pattern_matching::{Animal, pattern_matching};
mod refrences;
fn main() {
    shadowing::shadowing();
    let jungle_king = Animal::Tiger(String::from("Sher Khan"));
    
    // Call the function again
     pattern_matching(jungle_king); 
     refrences::references();
     

}
