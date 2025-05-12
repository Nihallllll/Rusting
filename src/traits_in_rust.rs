pub fn traits_in_rust(){
    trait Shape{
       fn area(&self)->f64;
    }
    struct Rect{
        len:f64,
        wid:f64
    }
    struct Circle{
        rad:f64
    }
    impl Shape for  Rect{
        fn area(&self)->f64{
            return self.len*self.wid;
        }
    }
    const  PI:f64 = 3.14;
    impl Shape for Circle{
        fn area(&self)->f64{
            return PI*self.rad*self.rad;
        }
    }
    fn print_shape<T:Shape + std::fmt::Display>(s:T){
        println!("Shape is {}",s);
    }
    let circle = Circle{
        rad:2.24
    };
    print_shape(circle);
}