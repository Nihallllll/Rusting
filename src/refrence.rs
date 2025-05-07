pub fn reference(){
    let mut x= 1;
    let y = &mut x;

    *y =  *y +1;//but * is not added automatically here 
    println!("{}",y);//in case of print we dont need *y ,coz rust does it automatically
}

//so & -> this means address , and * -> means whats in the address
//so x=5 , y=&x, so y= address of x and *y=value of the address it has 