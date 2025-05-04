pub fn owner() {
    let  a = String::from("hi");
    let b = a;
    // println!("{}",a);
    println!("{}",b);
}
pub fn string(){
    let name = String::from("nihal");
    println!("{}",name);
    let s = owner2(name);//name is now not the owner , its the s variable in the owner2
    
    println!("another one :{}",s);
}
pub fn owner2(s: String) -> usize{
    println!("this is from owner2 function :{}",s);
    return s.len();
}