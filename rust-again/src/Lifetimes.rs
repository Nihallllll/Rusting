fn longest_string<'a>(a :&'a str , b : &'a str ) -> &'a str{
    if a.len() > b.len(){
        return a;
    }else{
        return b;
    }
}

pub fn checker(){
    let a = "asddas";
    let ans ;
    {
      let b = "sads";
      ans = longest_string(&a,&b);
    }
     

    println!("the longest string is :{:}" , ans);
}