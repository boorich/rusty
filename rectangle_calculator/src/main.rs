#[derive(Debug)]  // usually makes sense to add for printing stuff without implementing custom print functions
struct Rectangle {
    width: u32,
    heigth: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {     // method
    self.width * self.heigth / 2
    }

    fn can_hold(&self, other: &Rectangle) -> bool {     // method
    self.width > other.width && self.heigth > other.heigth
    }

    fn square(size: u32) -> Rectangle {     // associated function, doesn't take self parameter
    Rectangle { width: size, height: size } // used here as constructor for new instances
    }
}

fn main() {
    let rect = Rectangle {width: 30, heigth: 50};
    let rect2 = Rectangle {width: 30, heigth: 50};

    // calling associated function square with :: syntax
    // something like this happens e.g. inside String::from("bla")
    // :: means that a function is namespaced by the associated struct
    let sq = Rectangle::square(3);

    println!("Rect is: {:#?}", rect);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!(
        "Can rect hold rect2? {}",
        rect.can_hold(&rect2)
    );


}
