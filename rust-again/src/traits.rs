trait Prints{
    fn prints(&self);
}

struct Rust;
struct C;

impl Prints for Rust{
    fn prints(&self){
        println!("Rust says println");
    }
}

impl Prints for C{
    fn prints(&self){
        println!("C says printf");
    }
}
fn lang_speak(l : &impl Prints){
    l.prints();
}

pub fn traits(){
   let a =  Rust;
   let b =  C;

   lang_speak(&a);
   lang_speak(&b);
}