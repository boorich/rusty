fn main() {
    let mut s1 = String::from("hello"); // ownership to s1

    let len = calculate_length(&s1); // refenrence aka pointer to the ownership into calculte_length()
                                     // which allows to access a value without having to take ownership

    println!("The length of {} is {}",s1, len);

    change(&mut s1);

    let len = calculate_length(&s1);
    println!("The length of {} is {}",s1, len);

    // only one mutable reference per scope allows rust to prevent data races at compile time.
    // A new scope can be introduced with curly braces anytime to separate refrences like so

    let mut b = String::from ("Hund");

    { // separating mutatable references with own scope
        let r1 = &mut b;
    }

    let r2 = &mut b;

    // mutable and immutable references are allowed to exist in a program if their scopes don't overlap
    // overlapping scopes occur if a immutable reference to data is used AFTER a mutable reference has
    // been introduced within the same pair of curly braces.
}

fn calculate_length(s: &String) -> usize { // referenced string explicit here
    s.len()
} // Here s goes out of scope but because only a reference was passed, s remains valid. This is called
  // borrowing.

// De-referencing is achieved by using the * operator

// fn change(some_string: &String) {
    // trying to modify a borrowed value
    // some_string.push_str(" world") // this fails since references don't own on default

 fn change(some_string: &mut String) {
    // modifying a borrowed value is alowed as long as there is only 1 mutable reference to a piece of data in 1 scope
    some_string.push_str(" world") // this works
 }

 // dangling references (referenced that point to memory that has already been dealloceated from a variable (dropped))
 // are directly addressed by the compiler with error[E0106]missing lifetime specifier