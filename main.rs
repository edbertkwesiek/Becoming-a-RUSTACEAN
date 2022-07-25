use std::io;
use std::io::Read;
use std::fs::File;


fn main() {
       let value = 64; 
       let read_value = &64;

       fn read_value(value:i32) -> Result<i32, io::Error>{
              let value:i32 = match value.trim().parse() {
                     Ok(value > 1 || value < 100) => Appropriate   
                     Err(value < 1 || value > 100) => panic!("Guess value must be between 1 and 100")
              };

       }

}

fn  read_username_from () -> Result<String, io::Error> {
       let f = File::open("hello.txt"); 

       let mut f = match f {
              Ok(file) => file, 
              Err(e) => return Err(e), 

       };
       let mut s = String::new(); 
       match f.read_to_string(&mut s) {
              Ok(_) => Ok(s), 
              Err(e)=> Err(e), 
       }


}