use std::io;
// Ordering is enum with the variants Less, Greater, and Equal.
use std::cmp::Ordering;
// The Rng trait defines methods that random number generators implement
use rand::Rng;

fn main() {

    // --snip--

    println!("Guess the number!");
    //rand::thread_rng function gives us the particular random number generator that we’re going to use which
    // is one that is local to the current thread of execution and seeded by the OS
    // then call gen_range which is defined by Rng trait from import
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    loop  {
        println!("Please input your guess.");

        // :: means "associated function of" the type left to the ::
        // An associated function is implemented on a type, in this case String,
        // rather than on a particular instance of a String. Some languages call this a static method
        let mut guess = String::new();

        // without importing std it could be std::io::stdin()
        // stdin() retuns an instance of std:io:Stdin which is a handle to the buffer of the standard input stream
        // read_line is called and whatever is typed into the keyboard is written to guess strin we're passing read_line
        // the & indicates that this is a reference to the memory so multiple parts of the code can access the same bytes
        io::stdin().read_line(&mut guess)
        // expect is used to check for potential failure
        // this works because read_line has a return type of "Result" which are enums.
        // For Result, the variants are Ok or Err. The Ok variant indicates the operation was successful, and inside Ok is
        // the successfully generated value. The Err variant means the operation failed, and Err contains information
        // about how or why the operation failed.
            .expect("Failed to read line");

        // this guess is the original guess variable taken in an shadowed into the new guess (common when casting types)
        // new guess variable is annotated (forced) to be u32 (unsigned 32-bit number)
        // trim cuts any leading or trailing whitespaces from the input stream
        // parse makes makes some kind of number from a strin and returns a result type (Err, Ok)
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number!");
        
        println!("You guessed: {}", guess);

        // the cmp method compares any two values and returns a variant ot the Ordering enum
        // match decides which pattern is fulfilled and runs that arm. Like a guard in Haskell.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                // Exiting the loop also means exiting the program, because the loop is the last part of main.
                break;
            }
        }
    }
}
