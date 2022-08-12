use std::io::Write;
use std::path::{PathBuf};
use std::process;

use structopt::StructOpt;
use anyhow::{Context, Error, Result};
use colored::Colorize;

use crate::exec::check_content;
use crate::cli::{RunOptions, Cli};

mod cli;
mod diff;
mod exec;
mod errors;

const FMT_TOKEN: &str = "{}";

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

/// because format! is a lil b-word, this manually formats it with a given numberx
fn dir_file_fmt(str: &str, num: u32) -> String {
    str.replace(FMT_TOKEN, &num.to_string())
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
            let default = "{}.in".to_string();
            let fin_fmt = args.fin_fmt.as_ref().unwrap_or(&default);
            let default = "{}.out".to_string();
            let fout_fmt = args.fout_fmt.as_ref().unwrap_or(&default);

            let once = fin_fmt.matches(FMT_TOKEN).count() == 0
                && fout_fmt.matches(FMT_TOKEN).count() == 0;

            let mut t = 1;
            loop {
                let fin_name = dir_file_fmt(fin_fmt, t);
                let fout_name = dir_file_fmt(fout_fmt, t);

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
                if once {
                    break;
                }
            }
        },
        _ => {
            eprintln!("{:?} and {:?} should either both be directories or files", args.fin, args.fout);
            process::exit(1);
        }
    };

    Ok(())
}
