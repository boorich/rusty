fn main() {
    let mut s = String::from("Hello 学中文"); // String owns the entire vector which represents the String
    // let s = "Hello 学中文"; // &str = pointer to a slice of a string would suffice if only read.
    println!("The length of the string in bytes here is: {}", s.len());
    println!("The amount of l's here is: {}", count_l(&s));
    s.push_str("more string with loads of llllll ...");
    println!("The amount of l's here is: {}", count_l(&s));

    for (i, c) in s.chars().enumerate() {     //s.chars() iterates through chars, enumerate wraps the result
        println!("{} = {}", i, c);
    }

    for (i, c) in s.char_indices() {     //s.char_indices() iterates through characters by index
        println!("{} = {}", i, c);
    }

    for (i, c) in s.bytes().enumerate() {     //s.bytes() iterates through bytes
        println!("{} = {}",i , c);
    }

    fn count_l(s: &str) -> i32 {
        let mut res = 0;
        for i in s.chars() {
            if i == 'l' {
                res += 1;
            }
        }
        res
    }
}
