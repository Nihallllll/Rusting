pub fn sum<T : std::ops::Sub<Output = T>>(a: T , b : T) -> T{
   return a - b;
}
pub fn res(){
    let res = sum(2,3);
    println!("{}",res);
}

// pub fn sum1<T, U>(a: T, b: U) -> f64
// where
//     T: std::convert::ToPrimitive,
//     U: std::convert::ToPrimitive,
// {
//     let a_float = a.to_f64().unwrap_or(0.0);
//     let b_float = b.to_f64().unwrap_or(0.0);
//     a_float + b_float
// }

// pub fn res() {
//     let result = sum(2, 3);
//     println!("{}", result); // Output: 5.0
//     let result2 = sum(2.5, 3);
//     println!("{}", result2);
// }
