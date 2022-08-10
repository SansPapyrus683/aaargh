use std::io::Write;
use structopt::StructOpt;
use std::path::{PathBuf};
use anyhow::{Context, Error, Result};
use colored::Colorize;
use crate::errors::ExecError;

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

    /// some graders don't care how you space your numbers
    /// if your grader isn't one of these, set this flag
    #[structopt(long = "whitespace-fmt")]
    whitespace_matters: bool,

    /// when comparing strings, should capitalization & the like matter?
    #[structopt(long = "str-case")]
    str_case: bool,

    /// should the programs output the stdout w/ the results?
    #[structopt(long = "prog-stdout")]
    prog_stdout: bool,

    /// should the programs output the stderr w/ the results?
    #[structopt(long = "prog-stderr")]
    prog_stderr: bool
}

fn path_test(path: &PathBuf) -> Result<(), errors::PathNotFound> {
    if path.exists() {
        return Ok(())
    }
    Err(errors::PathNotFound { path: path.clone() })
}

fn get_output(
    code: &PathBuf, input: &str,
    fin: &Option<PathBuf>, fout: &Option<PathBuf>
) -> Result<(String, String), Error> {
    match fin {
        None => {}
        Some(dir) => {
            let mut input_file = std::fs::File::create(dir)
                .expect("input file creation failed");
            for l in input.lines() {
                let eol = '\n';
                let mut l = l.to_string();
                l.push(eol);
                input_file.write_all(l.as_bytes())
                    .expect("writing to input file failed");
            }
        }
    }

    let res = exec::exec(code, Some(input))
        .with_context(|| format!(
            "error when executing {}", exec::path_str(code)
        ));

    return match res {
        Ok(ref out) => {
            match fout {
                None => res,
                Some(f) => {
                    let ans = std::fs::read_to_string(f).expect("crap");
                    let stderr = out.1.clone();
                    Ok((ans, stderr))
                }
            }
        }
        Err(_) => res
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();

    // testing if all the paths exist
    for p in vec![&args.code, &args.fin, &args.fout] {
        path_test(p)?;
    }

    match (args.fin.is_file(), args.fout.is_file()) {
        (true, true) => {
            let out = get_output(
                &args.code,
                &exec::check_content(&args.fin).unwrap(),
                &args.prog_fin, &args.prog_fout
            )?;

            if args.prog_stderr {
                println!("stderr output:");
                println!("{}", out.1.blue());
            }
            if args.prog_stdout {
                println!("stdout output:");
                println!("{}", out.0.green());
            }

            let fout_exp: String = exec::check_content(&args.fout)?;
            diff::diff_lines(
                out.0.lines().into_iter(),
                fout_exp.lines().into_iter(),
                args.whitespace_matters, args.str_case,
                &mut std::io::stdout()
            );
        },
        (false, false) => {
            todo!()
        },
        _ => {
            eprintln!("{:?} and {:?} should either both be directories or files", args.fin, args.fout);
            std::process::exit(1);
        }
    };

    Ok(())
}
