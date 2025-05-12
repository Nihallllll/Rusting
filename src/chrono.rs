use chrono::{Utc,Local};
pub fn print_time(){
let universe_time = Utc::now();
let local_time = Local::now();
println!("Global time is : {}",universe_time);
println!("India time is : {}",local_time);

}