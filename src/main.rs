use std::io::Write;
use std::path::{PathBuf};
use std::process;

use structopt::StructOpt;
use anyhow::{Context, Error, Result};
use colored::Colorize;

use crate::exec::{check_content, exec, ProgRes};
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
) -> Result<(ProgRes, String), Error> {
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

    let res = exec::exec(code, input, &options, compiled)
        .with_context(|| format!(
            "error when executing {}", exec::path_str(code)
        ));

    return match res {
        Ok(res) => {
            match fout {
                None => Ok((res, "".to_string())),
                Some(f) => {
                    let file = std::fs::read_to_string(f).expect("crap");
                    Ok((res, file))
                }
            }
        }
        Err(err) => Err(err)
    };
}

fn prog_res(
    res: &ProgRes,
    stdout: bool, stderr: bool,
    mut out: impl Write
) {
    if stderr {
        writeln!(out, "stderr output:\n{}", res.stderr.blue()).expect("oh no");
    }
    if stdout {
        writeln!(out, "stdout output:\n{}", res.stdout.green()).expect("oh no");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();

    let run_options = match args.run_options {
        None => RunOptions::None,
        Some(args) => args
    };

    path_test(&args.code)?;

    if args.gen.is_some() {
        let gen_code = args.gen.unwrap();
        for t in 1..=10 {
            let tc = get_output(
                &gen_code, "",
                &RunOptions::None, t != 1,
                &None, &None
            )?.0.stdout;  // discard stderr

            let correct = get_output(
                &args.ans, &tc,
                &run_options, t != 1,
                &args.prog_fin, &args.prog_fout
            )?.0.stdout;

            let (normal, file) = get_output(
                &args.code, &tc,
                &run_options, t != 1,
                &args.prog_fin, &args.prog_fout
            )?;

            println!("{}", format!("TEST CASE {}", t).cyan().bold());
            prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());
            println!("{}", format!("execution time (s): {}", normal.time).cyan());

            let ans = match args.prog_fout {
                None => normal.stdout,
                Some(_) => file
            };
            let diff_res = diff::diff_lines(
                ans.lines().into_iter(),
                correct.lines().into_iter(),
                args.whitespace_matters, args.str_case,
                &mut std::io::stdout()
            );

            if diff_res {
                println!("{}\n{}", "test case failed:".red(), tc.red());
                break;
            }
            println!("{}", "hooray, test case correct!".bright_green());
        }
        return Ok(());
    }

    let args_fin = args.fin.with_context(
        || format!("input file or directory not found")
    ).unwrap();
    let args_fout = args.fout.with_context(
        || format!("output file or directory not found")
    ).unwrap();

    // testing if all the paths exist
    for p in vec![&args_fin, &args_fout] {
        path_test(p)?;
    }

    match (args_fin.is_file(), args_fout.is_file()) {
        (true, true) => {
            let (normal, file) = get_output(
                &args.code, &check_content(&args_fin).unwrap(),
                &run_options, false,
                &args.prog_fin, &args.prog_fout
            )?;

            prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());

            let ans = match args.prog_fout {
                None => normal.stdout,
                Some(_) => file
            };
            let fout_exp = check_content(&args_fout)?;
            let diff_res = diff::diff_lines(
                ans.lines().into_iter(),
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
                let fin_name = fin_fmt.replace(FMT_TOKEN, &t.to_string());
                let fout_name = fout_fmt.replace(FMT_TOKEN, &t.to_string());

                let mut fin = args_fin.clone();
                fin.extend(&[fin_name]);
                let mut fout = args_fout.clone();
                fout.extend(&[fout_name]);

                if !fin.is_file() || !fout.is_file() {
                    break;
                }

                println!("{}", format!("TEST CASE {}", t).cyan().bold());

                let (normal, file) = get_output(
                    &args.code, &check_content(&fin)?,
                    &run_options, t > 1,
                    &args.prog_fin, &args.prog_fout
                )?;

                prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());

                let ans = match args.prog_fout {
                    None => normal.stdout,
                    Some(_) => file
                };
                let fout = check_content(&fout)?;
                let diff_res = diff::diff_lines(
                    ans.lines().into_iter(),
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
            eprintln!("{:?} and {:?} should either both be directories or files", args_fin, args_fout);
            process::exit(1);
        }
    };

    Ok(())
}
