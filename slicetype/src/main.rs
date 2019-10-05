// slices, like references don't have ownership and point to a portion of a String

// this extracts and returns the first word's ending index of a string with empty spaces 
// or the length of an entire if there are no spaces 
fn main() {
    let length = first_words_bad_practice(&String::from("Hello World"));
    println!("the length of the first string is: {}", length.to_string());
    let array = [1 ,2, 3, 4, 5, 6, 7, 8, 9, 0];
    // let rangearray = [9..18]; // using this causes panick
    // let hw = &String::from("Hello World"); // this works
    let openslice = &array[4..]; // [5, 6, 7, 8, 9, 0]
    let slice = &openslice[2..4]; // [7, 8]
    println!("The slice now contains: {:?}", slice); //debug print so value of slice can be displayed
}

// this is error prone, as it detaches the return value length from the state of the string
// e.g. if string gets erased with clear() method, the value won't be synced. Not quite an API.
fn first_words_bad_practice(s: &String) -> usize { // DANGER!! Return is separate value from input
    let bytes = s.as_bytes(); // convert String to an array of bytes using the as_bytes method

    for (i, &item) in bytes.iter().enumerate() { // iter is a method that returns each element in a
                                                 // collection and enumerate wraps the result of
                                                 // iter and returns each element as part of a tuple
        if item == b' ' {                        // as soon a there is a blank space (byte literal syntax)
            return i;                            // return the index
        }
    }

    s.len() //if there is no space in the s, return the lenght of entire string
}

// Example of slices
// let s = String::from("hello world");
// let hello = &s[0..5];
// let world = &s[6..11];

// Range syntax
// 1..2;   // std::ops::Range
// 3..;    // std::ops::RangeFrom
// ..4;    // std::ops::RangeTo
// ..;     // std::ops::RangeFull
// 5..=6;  // std::ops::RangeInclusive
// ..=7;   // std::ops::RangeToInclusive

// Much more robust version of first_words() making use of slices
// Whenever this is called it will return a corresponding slice
fn first_words(s: &String) -> &str {    // type that signifies “string slice” is written as &str
                                        // input and return values stay in sync as they are portions
                                        // the same data
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {       // as soon as there is a blank
            return &s[0..i];    // return a that slice up to the index
        }
    }

    &s[..]                      // return the entire string
}