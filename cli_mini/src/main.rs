use structopt::StructOpt;

// search for a pattern in a file and disopax the lines that contains it.
#[derive(StructOpt, Debug)]
struct cli {
    // pattern to look for 
    pattern: String,
    // path where file to read lives
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf, // like string but for system paths and works cross-platform
}

fn main() {
    let args = cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .expect("could not read file");
    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
