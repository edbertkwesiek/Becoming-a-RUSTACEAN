#[repr(align(n))]
struct Foo{
    tiny: bool,
    normal :u32,
    small:u8,

}

fn  replace_with_84( s: &mut Box<i32>){
    let  was = std::mem::take(s);

    *s = was ;
    let mut g = &s;
    let z = &mut g; 
    let mut r  = Box::new(84);
    std::mem::swap(s, &mut r);

    let mut x = Box::new(42);
    let  r =&x; 
    if rand() > 0.5 {
        * x =84;

    }else { 
        println!(" {} ", r);
    }
    let x = Box::new(i);


}
let mut s = Box::new(42);
replace_with_84(&mut s );



