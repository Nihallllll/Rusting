mod hello;
mod add;
mod vector;
mod bool;
mod ownership;
mod borrowing;
mod mutable_ref;
// mod struct_and_impl;
mod _impl;
fn main() {
   hello::say_hello();
   add::add(4, 5);
   bool::is_even(3);
   vector::vector();
   ownership::string();
   borrowing::string();
   borrowing::string1();
   mutable_ref::print();
   _impl::give_pbh(2, 3, 4);
   
}
