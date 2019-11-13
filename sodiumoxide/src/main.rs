use sodiumoxide;
use sodiumoxide::init;

fn main() {
    let res = init(); // inititalize sodiumoxide's fast primititives and returns `Ok` if initialization succeeded and `Err` if it failed.
    // let res = String::from("Deine Mudder");
    check(res);
}

fn check(res: Result<(), ()>) {
    match res {
        Ok(_) => println!("got the T"),
        Err(_) => println!("got the Err"),
    }
}

/*

match fs::metadata(path) {
    Ok(_) => assert!(false, "db file should not exist"),
    Err(_) => assert!(true),
}

    `init()` initializes the sodium library and chooses faster versions of
    the primitives if possible. `init()` also makes the random number generation
    functions (`gen_key`, `gen_keypair`, `gen_nonce`, `randombytes`, `randombytes_into`)
    thread-safe

    `init()` returns `Ok` if initialization succeeded and `Err` if it failed.
pub fn init() -> Result<(), ()> { // either something succeeded, or it failed, with nothing more to report in either case
    if unsafe { ffi::sodium_init() } >= 0 {
        Ok(())
    } else {
        Err(())
    }
} 

How to check:

let x: Result<i32, &str> = Ok(-3);
assert_eq!(x.is_ok(), true);

let x: Result<i32, &str> = Err("Some error message");
assert_eq!(x.is_ok(), false);

*/