use std::fmt::format;
use structopt::StructOpt;
use std::path::PathBuf;
use anyhow::{Context, Result};
use std::process::Command;
use colored::Colorize;

pub mod diff;
pub mod fcheck;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    // verbose: bool,
    #[structopt()]
    code: PathBuf,

    #[structopt(long = "fout")]
    fout: Option<PathBuf>,

    #[structopt(long = "fout-exp")]
    fout_exp: Option<PathBuf>,

    #[structopt(long = "whitespace-fmt")]
    whitespace_matters: bool
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    fcheck::check_content(&args.code)?;
    let lang = fcheck::file_lang(&args.code)
        .with_context(|| format!(
            "file {} is of an unknown language", fcheck::path_str(&args.code)
        ))?;

    if let Some(f) = &args.fout {
        if let Some(f_exp) = &args.fout_exp {
            let fout: String = fcheck::check_content(f)?;
            let fout_exp: String = fcheck::check_content(f_exp)?;

            diff::diff_lines(
                fout.lines().into_iter(),
                fout_exp.lines().into_iter(),
                args.whitespace_matters,
                &mut std::io::stdout()
            );
        }
    }

    Ok(())
}
