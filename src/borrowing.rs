pub fn string(){
    let str = String::from("Nihal");
    let (_len ,str) = get_len(str);
    println!("{}",str);
}
fn get_len(s : String) -> (usize,String){
    let _len= s.len();
    return (s.len(),s);
}

//also using refrence
pub fn string1(){
    let str = String::from("Nihal");
    let len = get_len1(&str);
    println!("this is using & : {}",str);
}
fn get_len1(s : &String) -> usize{
    
    return s.len();
}
