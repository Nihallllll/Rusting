pub fn tuple(){
    let  animal:(&str,u8) = ("Tiger",4);
    

    //tuple destructuring 
    let (name , legs)= animal;
    println!("animal name :{} ,number of legs : {}",name,legs);
}