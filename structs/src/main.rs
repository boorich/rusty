// Structs and enums are the building blocks for creating new types in
// a program’s domain to take full advantage of Rust’s compile time type checking.

fn main() {

    // instatiating a mutable user1
    let mut user1 = User { // mut can only be applied to entire struct, not individual fields
        // in structs, unlike tuples, order does not matter. Work like a dictionaries.
        email: String::from (""),
        username: String::from(""),
        sign_in_count: 1,
        active: true,
    };

    let mut player1 = Player {
        name: "Martin".to_string(),
        age: 40,
        height: 185,
        shoesize: 43,
    };

    // some input to change the instance's value
    println!("Please input your name:");
    std::io::stdin().read_line(&mut user1.username).expect("Failed to read line");

    println!("Please input your email:");
    std::io::stdin().read_line(&mut user1.email).expect("Failed to read line");

    // debug print with {:#?}
    println!("*** Debug Print ***");
    println!("The User 1 struct from main() includes these values: {:#?}", user1);

    // regular print with {}
    println!("The username of User 1 from main()is: {}", user1.username);

    // custom print with simple_print()
    println!("The simple print of Player 1 is: {}", player1.simple_print()); // since simple_print takes &self as an argument it refers to player1
    player1.grow(20);
    println!("The simple print of grown Player 1 is: {}", player1.simple_print());
    println!("The simple print of User 1 is: {}", user1.simple_print());

    let user2 = build_user(String::from("Doctor Love"), String::from("some@mail.com"));
    println!("*** Debug Print ***");
    println!("The User 2 struct from build_user() includes: {:#?}", user2);
    println!("The username of User 2 from build_user()is: {}", user2.username);

    // updating a struct with struct update syntax
    let user3 = User {
        email: String::from ("Inspector Gadget"),
        username: String::from("other@mail.com"),
        ..user2 // this is shorthand for "take anything else from user2"
    };
    println!("*** Debug Print ***");
    println!("The User 3 struct from main() includes: {:#?}", user3);
    println!("The username of User 3 from main()is: {}", user3.username);

    let tuple = make_tuple_struct();
    println!("*** Debug Print ***");
    println!("The tuple struct from main() includes: {},{},{}", tuple.0, tuple.1, tuple.2);

    player1.die();
    // player1.die(); not accisble here anymore since player1 was moved into player1.die()    
}

// a struct grouping user data to be instatiated
#[derive(Debug)] // imnplements Debug Trait, so it can be easily printed later
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn simple_print(&self) -> String {
        format! (" {} - {} - {} - {}", self.username, self.email, self.sign_in_count, self.active)
    }
}

struct Player {
    name: String,
    age: i32,
    height: i32,
    shoesize: i32,
}

// everything that needs to be implemented onto the type of player goes here
impl Player {
    fn simple_print(&self) -> String {
        format! ("{} - {} - {} cm - shoe: {}", self.name, self.age, self.height, self.shoesize)
    }

    fn grow (&mut self, h: i32) {
        self.height += 20;
    }

    fn die (self) { // this actually consumes self and therefore destroys the calling struct
        println!("{} is now dead !!!", self.simple_print());
    }  
}

// function to instatiate users
fn build_user(username: String, email: String) -> User {
    User {
        username, // shorthand for username: username,
        email, // shorthand for email: email,
        sign_in_count: 1,
        active: true,
    } // this returns
}

// Tuple Structs when no names need to be associated with fields
// Ususally these make sense when de-constructed based on pattern matching
// Tuple struct instances behave like tuples: you can destructure them into their individual pieces,
// you can use a . followed by the index to access an individual value, and so on.

struct Color(i32, i32, i32);

fn make_tuple_struct() -> Color {
    let black = Color(0, 0, 0);
    black
}

// Unit Structs without any fields
// useful in situations in which you need to implement a trait on some type but don’t have any data
// that you want to store

struct Empty();
// let whatever = Empty;
