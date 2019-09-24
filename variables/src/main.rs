fn main() {
    let mut x = 5;
    println!("The assigned value of x is: {}", x);
    x = 6;
    println!("The re-asigned value of x is now: {}", x);
    
    let x = 4;
    println!("The shadowed value of x is: {}", x);
    
    let x = x + 1;
    println!("The 2nd shadowed value of x is: {}", x);
    
    let x = x * 2;
    println!("The 3rd shadowed value of x is: {}", x);

    const MX_VAL: u32 = 100_000;
    println!("The constant MX_VAL has got a vlaue of {}",MX_VAL);

    let y = 2; // f64
    println!("The float y has got a value of {}", y);

    let y: f32 = 3.0; // f32
    println!("The float y has got a value of {}", y);

    let b: f32 = 5.1;
    let z = b + y;
    println!("The float z has got a value of {}", z);

    // anaonymous way without storing in extra variabe
    println!("The remainder of b and y is {}", b % y);

    let ja = true; //implicit type
    let sicher_nicht: bool = false; //explicit type annotation
    println!("{} ist wahr und {} ist falsch", ja, sicher_nicht);

    // tuples
    // A tuple is a general way of grouping together some number
    // of other values with a variety of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup1 = (143, 23.0, -1);
    let tup2 = (234, 64.1, -17);
    
    // deconstruct tuples by pattern matching
    let (t11, t12, t13) = tup1;
    let (t21, t22, t23) = tup2;

    // direct access to elements
    // let t11 = tup1.0;
    
    println!("Element 1 von tup1 ist {}", t11);
    println!("Element 1 von tup2 ist {}", (tup2.0 as i64));
    println!("Summe von tup1 und tup2 {}", (t11 as f64) + (t12 as f64) + (t13 as f64) + (t21 as f64) + (t22 as f64) + (t23 as f64));

    // Arrays
    // Unlike a tuple, every element of an array must have the same type.
    // Arrays in Rust are different from arrays in some other languages because arrays in
    // Rust have a fixed length, like tuples.

    // simple array
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];
    
    // annotation of an array
    let array: [f32; 5] = [54.4, 53.79, 23.2, 79.6, 383.8];
    println!("Array enthält hier an Position 1: {}", array[0]);

    // autofill an array
     let array = [1; 5];
    // leads to [1, 1, 1, 1, 1]
     println!("Array enthält hier an Position 1: {}", array[0]);

    //this is out of bounds
    let index = 4;
    //this will panic if index is 5 and above
    let element = array[index];
    
    println!("The value of element is: {}", element);
}