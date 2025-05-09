use std::io;
pub fn mat_ch(){
    fn is_even(x:u32)->bool{
        if x%2==0{
            return  true;
        }else{
            return false;
        }
    }
    let mut num = String::new();
    
    println!("Plse enter the number:");
    io::stdin().read_line(&mut num).expect("Failed to take input");
    let num: u32 = match num.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };
    println!("you have taken {} as you input and is it even :-",num);
    match num{
        x if is_even(x)=>println!("true"),
        x if !is_even(x)=>println!("false"),
        _=>println!("Number not found")
    }
}