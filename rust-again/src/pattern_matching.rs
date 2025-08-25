
pub enum Animal {
    Fish(String),
    Tiger(String),
    Crocodile(String),
}

pub fn pattern_matching(a: Animal) {
    match a {
        Animal::Fish(jal) => println!("Machli jal ki rani h, {}", jal),
        Animal::Tiger(king) => println!("raja whi hoga jo hakdar hoga, {}", king),
        Animal::Crocodile(lizard) => println!("jalwa h hamara, {}", lizard),
    }
}