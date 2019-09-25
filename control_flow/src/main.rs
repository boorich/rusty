fn main() {
    let number = 16;
    
    // MUST evaluate to bool or leads to type mismatch
    if number < 15 {
        println!("Less than 15.");
    }
    else {
        println!("greater then 15");
    }

    // multiple conditions with elseif
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // assigning with condition
    let condition = true;
    let number = if condition {
        6 // same type or type mismatch
    } else {
        5 // same type or type mismatch
    };

    println!("The value of number is: {}", number);
}

// If you don’t provide an else expression and the condition is false, the program will just skip the if block
// and move on to the next bit of code.
