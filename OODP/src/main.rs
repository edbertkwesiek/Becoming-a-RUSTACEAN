fn reverse(pair:(i32, bool)) -> (bool, i32){

    // let can be used to bind the members of a tuple to variables
    let (int_param, bool_param) = pair;

    (bool_param , int_param)
}

// The following struct is for the activity 
use std::thread; 



#[derive(Debug)]

struct Matrix(f32, f32, f32, f32);

pub struct Mutex<T:?Sized><T:long_tuple>;{
    long_tuple(u8, u16,u32, i8, i16, i32, i64, f32, f64, bool)
}


    


    // A tuple with a bunch of different type

fn main() {
    //A tuple with a bunch of different types,

    let Long_Tuple:Mutex<T> = Mutex::<T>:: (1u8, 2u16, 3u32,  -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

    thread::scope(|s| {
        s.spawn(|| {
            println!(" length:{}", long_tuple.len());
        });
        s.spawn(|| {
             for n in &long_tuple{
                println!("{n}");
             }
            });
    });

}

// the issue here s we are looking for a heterogenous mutex  to be able to represent a mutable tuple 