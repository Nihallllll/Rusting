pub fn enum_with_val(){
    enum Shape{
        Circle(i32),
        Rectangle(i32,i32),
        Square(i32)
    }

    fn cal_area(s:Shape){
        match s{
            Shape::Circle(radius)=>println!("{:?}",radius*radius),
            Shape::Rectangle(l,b)=>println!("{}",l*b),
            Shape::Square(a)=>println!("{}",a*a)
        }
    }
    cal_area(Shape::Circle(5));
    cal_area(Shape::Rectangle(4, 6));
    cal_area(Shape::Square(3));
}