//use std::rc::Rc;
//use std::sync::{Arc, Mutex};
use std::mem;

// fn main() {
    //let a  = 10;
    //let b = Box::new(20);
   // let c = Rc::new(Box::new(30)); 
    //let d = Arc::new(Mutex::new(40));
    //println!( " a: {:?}, b: {:?}, c:{:?}, d:{:?}", a, b, c, d );
//}


//this function borrows a slice
fn analyze_slice(slice: &[i32]) {
    println!(" first element of the slice : {}", slice[0]);
    println!(" the slice has {} elements", slice.len());

}


fn main() { 
    let xs:[i32; 5 ] = [1, 2, 3, 4, 5 ];

    // All elements can be initialized into the same value 
    let _ys:[i32; 500] = [0; 500]; 

    // Indexing starts at 0 
    println!(" first element of the array: {} ", xs[0] );
    println!(" the second element of the array: {} ", xs[1]);
    println! (" the  number of elements in the array :{} ", xs.len());

    //Array are stack allocated 
    println!(" array occupies {} bytes ", mem::size_of_val(&xs));

    //Arrays can be automatically borrowed as slices 
    println!(" borrow the whole array as a slice ");
    analyze_slice(&xs);

    // slices can can be extracted from arrays;
    // they are of the form [ starting index and last or ending  index]
    // the starting index representing the first array item
    // the last one more last position referring to the ending index
    println!(" borrow a section of the array as a slice : {}" ); analyze_slice(&_ys[1..4]);

    // example of empty slice &[]
    let empty_array : [i32 ;  0 ] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[1..4]);

    // example of empty slice &[]
    let empty_array:[i32; 0] = [];
    assert_eq!(empty_array , &[]);
    assert_eq!(empty_array, &[][..]); 

    // arrays can be accessed with get 
    // get returns <Option >  and this option can be matched with  or works with .expect()
    // if you want the program to return happily 

    for i in 0..xs.len() + 1{
        match xs.get(i) {
            Some(xval) => println!(" {} : {}", i, xval ),
             None => println!("slow down! {} is too far!", i ),
        }
    }

    //out of bounds
    //println!(" xs[5] is : " xs[5]);

}
