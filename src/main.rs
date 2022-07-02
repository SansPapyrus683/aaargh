use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    // verbose: bool,
    #[structopt()]
    input_test_file: String,
    #[structopt()]
    output_test_file: String,
}

fn main() {
    let args = Cli::from_args();
    println!("aaargh {}", args.input_test_file);
}
