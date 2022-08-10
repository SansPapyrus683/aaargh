use structopt::StructOpt;
use std::path::{PathBuf};
use anyhow::{Context, Result};

mod diff;
mod exec;
mod errors;

#[derive(StructOpt)]
struct Cli {
    // #[structopt(short = "v", long = "verbose", help = "Print verbose output")]
    // verbose: bool,
    #[structopt()]
    code: PathBuf,

    /// file or directory to use for input
    #[structopt(long = "fin")]
    fin: PathBuf,

    /// file or directory that contains the actual outputs
    #[structopt(long = "fout")]
    fout: PathBuf,

    /// file name to use for input (if `None`, stdin will be used)
    #[structopt(long = "prog-fin")]
    prog_fin: Option<PathBuf>,

    /// file name to detect for output (if `None`, stdout will be used)
    #[structopt(long = "prog-fout")]
    prog_fout: Option<PathBuf>,

    #[structopt(long = "whitespace-fmt")]
    whitespace_matters: bool
}

fn path_test(path: &PathBuf) -> Result<(), errors::PathNotFound> {
    if path.exists() {
        return Ok(())
    }
    Err(errors::PathNotFound { path: path.clone() })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // testing if all the paths exist
    for p in vec![&args.code, &args.fin, &args.fout] {
        path_test(p)?;
    }
    if let Some(p) = &args.prog_fin {
        path_test(&p)?;
    }
    if let Some(p) = &args.prog_fout {
        path_test(&p)?;
    }

    if args.fin.is_dir() != args.fout.is_dir() {
        eprintln!("{:?} and {:?} should either both be directories or files", args.fin, args.fout);
        std::process::exit(1);
    }

    let res = exec::Lang::exec(&args.code)
        .with_context(|| format!(
            "error when executing {}", exec::path_str(&args.code)
        ))?;

    dbg!(args.fout);

    // let fout: String = fcheck::check_content(&args.fout)?;
    // let fout_exp: String = fcheck::check_content(&args.prog_fout.unwrap())?;
    //
    // diff::diff_lines(
    //     fout.lines().into_iter(),
    //     fout_exp.lines().into_iter(),
    //     args.whitespace_matters,
    //     &mut std::io::stdout()
    // );

    Ok(())
}
