fn main() {

    let s = String::from("h, e, l,l,o ").len();

     

    takes_ownership(s);

    let x = 5;
    makes_copy(x);

    let a = [10, 20, 30, 40, 50 ];
    let mut index = 0;

    while index < 5 {
        println!(" the value is : {}", a[index]);
        index = index + 1; 
    }
}
fn takes_ownership(x : Option<String >) -> Option<u8> {
    match x {
        Some(String )  => println!(" it is  correct"),
        None => println!( " not correct"),

    }

}
fn makes_copy(some_integer :  i32) {  // some integer comes into scope 
    println!(" {} ", some_integer ); 

}



