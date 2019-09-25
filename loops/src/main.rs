fn main() {

    // a while loop
    let mut counter = 0;
    while counter < 10 {
        counter += 1;
        println!("The counter is: {}", counter);
    };

    // an infinite loop
    let rval = 12;
    
    loop {
        println!("Again");
        // forced to return something here
        break rval;
    };

    println!("The returned value is: {}", rval);
  
    // a for loop
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    // a nicer way to make a for loop including ranges and the reverse-function
    for number in (1..4).rev() {
    println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
