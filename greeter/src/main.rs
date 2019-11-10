use std::env::args;

// under the condition that checking for the first character of any given argument there is a "W"
// make a println! statement

fn main() {
    for a in args() {
        if let Some(c) = a.chars().next() { // chars() returns an iterator over the chars of a string slice.
                                            // next() advances the iterator and returns the next value.
            match c {
                'w' | 'W' => println!("Hello {}!", a), // is c a "w" or a "W"
                _ => {}                     // like continue
            }
        }
    }
}

/* My version

fn main() {
    for (_index, name) in args().enumerate() {
        let bytes = name.as_bytes();
        let name = &name;
        // println!("Hello {}!", bytes[0]);
        // println!("Hello {}!", name);

        if bytes[0] == 87 {
            println!("Hello {}!", &name) 
        }
        if bytes[0] == 119 {
            println!("Hello {}!", &name) 
        }
        else { continue; }
    }
}
*/






