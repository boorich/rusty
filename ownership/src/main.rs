// Ownership

// A memory-management system with a set of rules that the compiler checks at compile time.
// In Ruse allocated memory is automatically returned once the variable that owns it goes out of scope.

// Each value in Rust has a variable that’s called its owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Simple types go on the stack, more complex types go on the heap and a pointer on the stack
// Ownership is especially useful for data that gets allocated to the heap since it reduces the
// pain of dealing with issues that come from memory allocation and garbage collection

fn main() {
    
    let _s = "Hello "; // s's scope begins here | it now has and owner and is valid
    
    let mut a = String::from("Hello"); // here we create a mutatable String from a string literal using
    // the from function and namespace this specific from-function under the String type rather than using
    // some sort of name like string_from. Here memory is requested from the OS.

    a.push_str(", world!"); // push_str() appends a literal to a string
    println!("{}", a); // print the appended string

    // how variables and data interact
    let x = 5; // 5 binds to x | pushed on stack
    let _y = x; // a copy of x binds to y | pushed on stack

    // move operation
    let s1 = String::from("hello"); // puts a pointer to the memory location on the heap, a length and a capacity on the stack
    let _s2 = s1; // puts a copy of s1 (pointer and stuff) onto stack and both now point to same memory
    // when variables containing pointer go out of scope they are dropped. The heap is only freed if not pointer are still valid
    
    // clone operation
    let s1 = String::from("hello");
    let s2 = s1.clone(); //this actually does make a copy on the heap, but it a fairly expensive operation. Avoid.

    //copy operation
    let x = 5;
    let y = x; 

    println!("x = {}, y = {}", x, y); // x is still valid because types with known size like e.g. int have a copy trait which
    // puts them on the stack instead of the heap. These types cannot have the drop trait. 

println!("s1 = {}, s2 = {}", s1, s2);
}   // here s's owner goes out of scope | s's value is dropped and it is no longer valid | Rust calls drop() here automatically
