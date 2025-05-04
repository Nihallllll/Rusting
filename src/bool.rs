pub fn is_even(a: u32) -> bool {
    let result = a % 2 == 0;
    println!("Is {} even? {}", a, result);
    result
}
