
#[allow(unused_imports)]
use std::pin::Pin;

enum RectangleKind{
    Trapezium(u32),
    Trapezoid(String),


}
struct Rectangle{
    width: u32,
    length:u32,

    width1: u32,
    width2: u32,

    l1 : u32,
    l2: u32,
}
struct TypeofQuadrilateral{
    Quadrilateral1 : RectangleKind,
    NameofQuadrilateral : String,
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width  * self.length 

    }
    fn perimeter(&self) -> u32{
        self.width1 + self.width2
    }
    fn square(&self) -> u32{
        self.l1 * self.l2
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width  && self.l1 > other.l2
    }
}

impl Rectangle{
    fn square1(  size: u32) -> Self{
        Self{
            width : size,
            length : size,
            width1 : size,
            width2 : size,
            l1 : size,
            l2 : size, 
        }
    }
}
fn main(){
    let rect1 = Rectangle {
        width: 30,
        length : 50,
        width1 : 20,
        width2 : 15,
        l1: 3,
        l2:16,
    };
    let rect2: Rectangle{
        width: 30,
        length: 16,
        width1: 24,
        width2 : 15,
        l1: 3,
        l2: 4, 


    };
    let sq = Rectangle::square(3);
    let enum1 = TypeofQuadrilateral{
        Quadrilateral1:Rectanglekind::Trapezium(50),
        
    
    }

    println!( 
        "the are of the rectangle is  {} ", rect1.area()
    );
    println!( " the perimeter is {}", rect1.perimeter());
    println!( " the square is {}", rect1.square());
    println!( "  can rect1 hold rect2", rect1.can_hold(&rect2));
    println!(" the value of the square is ", sq.square1)
}