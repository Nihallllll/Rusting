use crate::slices::slices;





mod hello;
mod add;
mod vector;
mod bool;
mod ownership;
mod borrowing;
mod mutable_ref;
// mod s_and_t;
mod _impl;
mod tuple;
mod refrence;
mod array;
mod shadowing;
mod for_loop;
mod mat_ch;
mod input_in_rust;
mod enum_with_values;
mod defined_enums;
mod chrono;
mod dotenv_env;
mod generics_and_traits;
mod generic_and_enums;
// mod traits_in_rust;
mod loops;
mod slices;
mod serde;
mod Brosh;
fn main() {
   hello::give_pbh(2, 3, 5);
   add::add(4, 5);
   bool::is_even(3);
   vector::vector();
   ownership::string();
   borrowing::string();
   borrowing::string1();
   mutable_ref::print();
   _impl::give_pbh(2, 3, 4);
   tuple::tuple();
   refrence::reference();
   array::arr();
   shadowing::shadow();
   for_loop::for_loop();
   mat_ch::mat_ch();
   // input_in_rust::mat_ch();
   enum_with_values::enum_with_val();
   defined_enums::defined_enums();
   chrono::print_time();
   dotenv_env::read_env_file();
   generics_and_traits::res();
   generic_and_enums::generics_and_enums();
   // traits_in_rust::traits_in_rust();

   struct User {
      name: String ,
      
   }
   impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "User: {}", self.name)
    }
}

   let u1 = User{
      name :String::from("nihal")
   };
   let u2 = User{
      name :String::from("Rajak")
   };

   fn display<T: std::fmt::Display>(a:T , b:T){
      print!("{}",a);
      print!("{}",b);
   }
   display(2, 3);
   display(u1, u2);

   loops::loops();

   let mut s = String::from("helloworld");
   let res = slices(&s);
   
   println!("the string is {s} and the value is {res}");
   s.clear();

   
    println!("{}","Here are macros");
    #[derive(Debug)]
    struct User2 {
        name: String,
        age: u32
    }
    

    let u = User2 {
       name : String::from("Nihal"),
       age: 22 
    };
    
    println!("{:?}",u);
    

    serde::serde();
    Brosh::borsh_example();
}
