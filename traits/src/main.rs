extern crate rand;

use rand::Rng;  //Rng is a trait from the rand object

use std::ops::Add;  // import Add first to implement an override function

// Traits are used like typeclasses in Haskell

#[derive(Debug)]
pub struct Point {
    x:i32,
    y:i32,
}

impl Point {
    fn random() -> Self {
        let mut tr = rand::thread_rng();
        Point{
            x:tr.gen(),
            y:tr.gen(),
        }
    }
}

impl Add for Point {
    type Output = Point;        // this could be a totally different type at times
    fn add (self, other:Point) -> Self::Output { //Self with capital S means "The type that self is"
        Point {                 // add Points like this
            x:self.x + other.x,
            y:self.y + other.y,
        }
    }
}

fn main() {
    let a = Point{x:3, y:5};
    println!("A = {:?}", a);
    let b = Point{x:30, y:50};

    let c = a + b;
    let d = Point::random();

    println!("C = {:?}", c);
    println!("D = {:?}", d);
}
