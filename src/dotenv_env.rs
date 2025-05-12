use dotenv::dotenv;
use std::{env, str};

pub fn read_env_file(){
    dotenv().ok();
    let read = env::var("Data_base_url");
    
    match read {
        Ok(url)=>println!("the data base url is : {:?}" , url),
        Err(e)=>println!("the data base url is faulty")
    }
}