use structopt::StructOpt;
use colored::*;

pub mod fcheck;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    // verbose: bool,
    #[structopt()]
    input: String,
    #[structopt()]
    code: String,
    #[structopt()]
    output: String,
}

fn main() {
    let args = Cli::from_args();
    println!("aaargh {}", args.input);
    dbg!(fcheck::file_lang(&args.code));
    if fcheck::file_ext(&args.code) == Some("cpp") {
        println!("ok cool");
    }
}
