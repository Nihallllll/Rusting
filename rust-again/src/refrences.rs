pub fn references(){
    let mut str = String::from("Nihal");
    let ref1 = &mut str;
    ref1.push_str("Rajak");
    let ref2 = &str;
    
    println!("{}", ref2);
}