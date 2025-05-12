pub fn mat_ch(){
    fn is_even(x:u32)->bool{
        if x%2==0{
            return  true;
        }else{
            return false;
        }
    }
    let num = 5;
    match num{
        y if is_even(y)=>println!("true"),
        x if !is_even(x)=>println!("false"),
        _=>println!("Number not found")
    }
}