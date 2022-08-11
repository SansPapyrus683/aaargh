use std::ffi::{OsStr, OsString};
use std::io::Write;
use structopt::StructOpt;
use std::path::{PathBuf};
use std::process;
use anyhow::{Context, Error, Result};
use colored::Colorize;
use crate::exec::check_content;

mod diff;
mod exec;
mod errors;

#[derive(StructOpt)]
struct Cli {
    /// code file (only supports c++, py, and java)
    #[structopt()]
    code: PathBuf,

    /// file or directory to use for input
    #[structopt(long = "fin")]
    fin: PathBuf,

    /// file or directory that contains the actual outputs
    #[structopt(long = "fout")]
    fout: PathBuf,

    /// note: won't be used if `fin` & `fout` are normal files
    /// the format string for the input files
    /// (occurrences of `{}` will be replaced with numbers starting from 1)
    #[structopt(long = "fin-fmt")]
    fin_fmt: Option<String>,

    /// the format string for the output files (basically same thing as `fin_fmt`)
    #[structopt(long = "fout-fmt")]
    fout_fmt: Option<String>,

    /// file name to use for input (if `None`, stdin will be used)
    #[structopt(long = "prog-fin")]
    prog_fin: Option<PathBuf>,

    /// file name to detect for output (if `None`, stdout will be used)
    #[structopt(long = "prog-fout")]
    prog_fout: Option<PathBuf>,

    /// some graders don't care how you space your numbers.
    /// if your grader isn't one of these, set this flag
    #[structopt(long = "whitespace-fmt")]
    whitespace_matters: bool,

    /// when comparing strings, should capitalization & the like matter?
    #[structopt(long = "str-case")]
    str_case: bool,

    /// should the programs output the stdout w/ the diff results?
    #[structopt(long = "prog-stdout")]
    prog_stdout: bool,

    /// should the programs output the stderr w/ diff the results?
    #[structopt(long = "prog-stderr")]
    prog_stderr: bool,

    #[structopt(subcommand)]
    run_options: Option<RunOptions>
}

// https://docs.rs/structopt/latest/structopt/#external-subcommands
#[derive(Debug, PartialEq, StructOpt)]
pub enum RunOptions {
    None,
    #[structopt(external_subcommand)]
    Some(Vec<OsString>)
}

fn path_test(path: &PathBuf) -> Result<(), errors::PathNotFound> {
    if path.exists() {
        return Ok(())
    }
    Err(errors::PathNotFound { path: path.clone() })
}

fn get_output(
    code: &PathBuf, input: &str,
    options: &RunOptions, compiled: bool,
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

    let res = exec::exec(code, Some(input), &options, compiled)
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

fn dir_file_fmt(str: &str, num: u32) -> String {
    let fmt_token = "{}";
    assert!(str.matches(fmt_token).count() > 0);
    str.replace(fmt_token, &num.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();

    let run_options = match args.run_options {
        None => RunOptions::None,
        Some(args) => args
    };

    // testing if all the paths exist
    for p in vec![&args.code, &args.fin, &args.fout] {
        path_test(p)?;
    }

    match (args.fin.is_file(), args.fout.is_file()) {
        (true, true) => {
            let prog_out = get_output(
                &args.code, &check_content(&args.fin).unwrap(),
                &run_options, false,
                &args.prog_fin, &args.prog_fout
            )?;

            if args.prog_stderr {
                println!("stderr output:");
                println!("{}", prog_out.1.blue());
            }
            if args.prog_stdout {
                println!("stdout output:");
                println!("{}", prog_out.0.green());
            }

            let fout_exp: String = check_content(&args.fout)?;
            let diff_res = diff::diff_lines(
                prog_out.0.lines().into_iter(),
                fout_exp.lines().into_iter(),
                args.whitespace_matters, args.str_case,
                &mut std::io::stdout()
            );

            if !diff_res {
                println!("{}", "hooray, test case correct!".bright_green());
            }
        },
        (false, false) => {
            if args.fin_fmt.is_none() || args.fout_fmt.is_none() {
                eprintln!("if using folders, a file format string must also be given");
                process::exit(1);
            }

            let mut t = 1;
            loop {
                let fin_name = dir_file_fmt(&args.fin_fmt.as_ref().unwrap(), t);
                let fout_name = dir_file_fmt(&args.fout_fmt.as_ref().unwrap(), t);

                let mut fin = args.fin.clone();
                fin.extend(&[fin_name]);
                let mut fout = args.fout.clone();
                fout.extend(&[fout_name]);

                if !fin.is_file() || !fout.is_file() {
                    break;
                }

                println!("{}", format!("TEST CASE {}", t).cyan().bold());

                let prog_out = get_output(
                    &args.code, &check_content(&fin)?,
                    &run_options, t > 1,
                    &args.prog_fin, &args.prog_fout
                )?;

                if args.prog_stderr {
                    println!("stderr output:");
                    println!("{}", prog_out.1.blue());
                }
                if args.prog_stdout {
                    println!("stdout output:");
                    println!("{}", prog_out.0.green());
                }

                let fout = check_content(&fout)?;
                let diff_res = diff::diff_lines(
                    prog_out.0.lines().into_iter(),
                    fout.lines().into_iter(),
                    args.whitespace_matters, args.str_case,
                    &mut std::io::stdout()
                );

                if !diff_res {
                    println!("{}", "hooray, test case correct!".bright_green());
                }

                t += 1;
            }
        },
        _ => {
            eprintln!("{:?} and {:?} should either both be directories or files", args.fin, args.fout);
            process::exit(1);
        }
    };

    Ok(())
}
