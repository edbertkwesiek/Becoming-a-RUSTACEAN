#![allow(non_snake_case)]
//   this is JUST about traits
#[allow(non_snake_case)]

use std::str;
struct Container<T>{ 
value: T 
}
impl<T>Container<T>{ 
    fn give_me(value: T ){
        let _ = value ;
    }
    
}
fn main() {
    let a = "generics";
    let b = 1024;


    let  give_me = 45;
    
    let num_from_str = str::parse::<u8>("34").unwrap();
    println!(" Parsed number {}", num_from_str);
}

