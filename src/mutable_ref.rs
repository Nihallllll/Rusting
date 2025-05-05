pub fn print(){
    let mut str = String::from("Nihal");
    let r1= &mut str;
    r1.push_str("Rajak");
    let r2 =&r1;
    println!("{}",str);
    // println!("this is r2 :{}",r2);

    //if line 7 is uncommented it gives error in str(line 6)coz r1,r2 are the boss until they have the ref
    //str cannot be used under the scope or code of r1 and r2 

    //  str= "xyx";
    //  r1=&str;
    //  {
    //     str cannot be used here 
    //  }
    //  here str can be used
}
// You can only have one immutable reference.
// If there is an immutable reference, there cant be other immutable or mutable references 
// You can have multiple immutable references