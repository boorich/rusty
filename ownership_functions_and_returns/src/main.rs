// In Rust The final expression in the function will be used as return value.
// Alternatively, the return statement can be used to return a value earlier
// from within the function, even from inside loops or ifs.

fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    let s4 = String::from("hello");     // s4 enters scope

    let (s5, len) = calculate_length(s4); // s4 moved into calculate_length(),
                                          // which gives back s5 and length

    println!("The length of {} is {}.",s5, len);

} // Here, s3, s5 and len goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// takes a string and returns a string and a pointer-sized unsigned integer type
fn calculate_length(string: String) -> (String, usize) { // this can be done more hassle-free
    let len = string.len(); // returns the length of a string

    (string, len) // last expression is returned | no semicolon here!
}
