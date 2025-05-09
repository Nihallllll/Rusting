use std::fs;
pub fn defined_enums(){
    let content = fs::read_to_string("a.txt");
    match content{
        Ok(content) => println!("{}",content),
        Err(e)=>println!("Error 404")
    }
}