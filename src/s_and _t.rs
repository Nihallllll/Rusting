    struct Triangle {
        p : u32,
        b : u32,
        h : u32
    }
    //we have to define the struct before hand to use
    impl Triangle{
        fn area(&self){
            
            println!("the area is : {}",self.p +self.b +self.h);
        }
        fn print(){
            println!("the area is : {}",121)
        }
    }
    pub fn give_pbh(a: u32,b: u32,c: u32){
    let triangle = Triangle{
        p: a,
        b: b,
        h: c
    };
    println!("height : {}, Base : {} , hypotenues : {}",triangle.p,triangle.b,triangle.h);
    println!("the area is : {}", triangle.area());
    }