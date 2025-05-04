mod hello;
mod add;
mod vector;
mod bool;
mod ownership;
fn main() {
   hello::say_hello();
   add::add(4, 5);
   bool::is_even(3);
   vector::vector();
   ownership::string();
   
}
