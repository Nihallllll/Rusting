use serde::{Deserialize,Serialize};
use serde_json::Number;
pub fn serde(){
    #[derive(Deserialize,Serialize)]
    struct  Dog{
       name : String,
       popularity : u16
    }

    let dog_name = Dog{
        name : String::from("Pug"),
        popularity : 15
    };

    let serialized_str = serde_json::to_string(&dog_name);
    println!("THe serde string  is : {:?}",serialized_str);
}