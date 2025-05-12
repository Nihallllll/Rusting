
pub fn generics_and_enums(){
    #[derive(Copy,Clone)]
    struct  Rect<T>{
        len : T,
        wid : T
    }
    impl <T :std::ops::Mul<Output = T>+ Copy> Rect<T>{
        fn area(&self)->T{
            return self.len * self.wid;
        }
    }
    let r = Rect {
       len: 2,
        wid:3
    };
    
    println!("len * wid is :{}",r.area());
}