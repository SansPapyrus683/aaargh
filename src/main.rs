use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::{Context, Result};
use std::process::Command;

pub mod diff;
pub mod fcheck;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    // verbose: bool,
    #[structopt()]
    code: PathBuf,

    #[structopt()]
    fout: PathBuf,

    #[structopt()]
    fout_exp: PathBuf,

    #[structopt(long = "whitespace-fmt")]
    whitespace_matters: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();
    let lang = fcheck::file_lang(&args.code);

    let fout: String = std::fs::read_to_string(&args.fout)
        .with_context(|| format!("where's {}", fcheck::path_str(&args.fout)))?;

    let fout_exp: String = std::fs::read_to_string(&args.fout_exp)
        .with_context(|| format!("where's {}", fcheck::path_str(&args.fout_exp)))?;

    diff::diff_lines(
        fout.lines().into_iter(),
        fout_exp.lines().into_iter(),
        args.whitespace_matters,
        &mut std::io::stdout()
    );

    Ok(())
}
