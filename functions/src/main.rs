fn main() {
    println!("Hello, world!");

    another_function(5, 6);
    express();
    println!("The value of x is: {}", five());

}

// Function could be defined anywhere as long as it is there.
// Argumentś type MUST always be annotated in a function definition.
fn another_function(x: i32, y: i32) {
    println!("The value passed into another_function are: {}  {}", x, y);
}

// statements do not return a value
// expressions return a value
fn express() {
    let y = {
        let x = 3;
        // no ending semicolon here = expression = returns value
        x + 1
    };

    println!("The value of y is: {}", y);
}
// return type specified as i32
fn five() -> i32 {
    // implicitly returns value of last expression in block 
    5
}

// about the ;
// anything that ends with a semicolon will retun a an empty tuple like so ()
// the empty tuple is Rust's way to decribe a void, None, nil, ...
// statements are for code that needs to "just run through" whereas expressions are ther to return something
