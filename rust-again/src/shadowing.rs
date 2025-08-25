pub fn shadowing() {
    let x = 10 ;
    let x = "Hello";

    println!("The x was 10 but got shadowed into : {}",x);
}