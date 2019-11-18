use structopt::StructOpt;

// search for a pattern in a file and disoplay the lines that contain(s) it.
#[derive(StructOpt, Debug)]
struct Cli {
    // pattern to look for 
    pattern: String,
    // path where file to read files
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,   // like string but for system paths and works cross-platform
                                // obsolete here, since the file is being set in result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args(); // digest 2 reqired args into struct
    let content = std::fs::read_to_string("test.txt")?; // The ? internally expands into a match to catch input errors
    for line in content.lines() { // find, extract and print pattern
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("file content: {}", content);
    Ok(())
}

/*
What is going on? Mid Version

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args(); // digest 2 reqired args into struct
    let result = std::fs::read_to_string("text.txt"); // read actual file 
    let content = match result {
        Ok(content) => { content },
        Err(error) => { return Err(error.into()); }
    };
    for line in content.lines() { // find, extract and print pattern
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    println!("file content: {}", content);
    Ok(()) // Default return. This is ok, do nothing.
}


What is going on? Long Version.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args(); // digest 2 reqired args into struct
    let result = std::fs::read_to_string("text.txt"); // this is supposed to be the file to consume
    // hint: shorthand for below block using unwrap panics
    // let content = std::fs::read_to_string("text.txt").unwrap();
    let content = match result { // guards that result is a good file
        Ok(content) => { content }, // if good put content into content
        Err(error) => { return Err(error.into()); } // if no or bad file, error
    };

    println!("file content: {}", content); // print content of the entire file
    Ok(());
    
    for line in content.lines() { // find, extract and print pattern
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
*/
