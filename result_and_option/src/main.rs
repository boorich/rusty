use std::collections::HashMap;
use std::env::args; //function that returns an iterator over the args that were supplied

fn main() {

    let mut hm = HashMap::new(); //instatiating a Hashmap which can infer itś type from what is put in first

    hm.insert(3, "Hello");
    hm.insert(5, "world");
    // Options allow to check for either something that we want or something that errors like so 
        // Some(v) => v,
        // _=> "Nothing here!",

    let r = hm.get(&3).unwrap();    // calling unwrap on hm.get(&3) will check and return a value

    println!("{}", r);
    
    // let r = hm.get(&4).unwrap();    // calling the same on (&4) will panick       
    let r = hm.get(&4).unwrap_or(&"NoString");    // unwrap_or mitigates panick

    println!("{}", r);

    match get_arg(3) {
        Ok(v)=>println!("OK - {}", v),
        Err(e)=>println!("Error - {}", e),
    }
}

fn get_arg(n:usize)->Result<String,String> { // args will be given when running cargo run [arg1] [arg2] ...
    for (i,a) in args().enumerate() {
        if i == n {
            return Ok(a);
        }
    }
    Err("Not enough Args".to_string())
}
