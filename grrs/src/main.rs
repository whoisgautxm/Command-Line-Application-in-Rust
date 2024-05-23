
use clap::Parser;

#[derive(Parser)]
struct CLI{
    /// The pattern to look
    pattern:String,
    /// The path to the file to read
    path : std::path::PathBuf,
}


fn main() {
    

    let args = CLI::parse();
    println!("pattern: {:?}, path:{:?}",args.pattern,args.path)
}


