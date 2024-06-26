use std::io::Write;
use std::path::PathBuf;

use structopt::StructOpt;
use anyhow::{Context, Error, Result};
use colored::Colorize;

use crate::exec::{check_content, exec, ProgRes};
use crate::cli::{RunOptions, Cli};
use crate::errors::ExecError;

mod cli;
mod diff;
mod exec;
mod errors;

const FMT_TOKEN: &str = "{}";

fn path_test(path: &PathBuf) -> Result<(), errors::PathNotFound> {
    if path.exists() {
        return Ok(());
    }
    Err(errors::PathNotFound { path: path.clone() })
}

fn get_output(
    code: &PathBuf, input: &str,
    options: &RunOptions, compiled: bool,
    fin: &Option<PathBuf>, fout: &Option<PathBuf>,
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
    mut out: impl Write,
) {
    if stderr {
        writeln!(out, "stderr output:\n{}", res.stderr.blue()).expect("oh no");
    }
    if stdout {
        writeln!(out, "stdout output:\n{}", res.stdout.green()).expect("oh no");
    }
}

fn validate(
    output: &String,
    ans: &Option<String>,
    checker: &Option<PathBuf>,
    compiled: bool,
    whitespace_matters: bool, str_case: bool, one_abort: bool,
    mut out: impl Write,
) -> Result<bool, ExecError> {
    if let Some(a) = ans {
        let diff_res = diff::diff_lines(
            output.lines().into_iter(),
            a.lines().into_iter(),
            whitespace_matters, str_case, one_abort,
            out,
        );
        return Ok(diff_res);
    }
    if let Some(c) = checker {
        let correct = exec::exec(
            &c, output,
            &RunOptions::None, compiled,
        );
        return match correct {
            Ok(o) => {
                if o.stdout.trim().to_lowercase() == "ok" {
                    return Ok(true);
                }
                writeln!(
                    out, "{}",
                    format!("incorrect output- checker message:\n{}", o.stdout).red()
                ).expect("oh no");
                Ok(false)
            }
            Err(e) => Err(e)
        };
    }
    Ok(true)  // PISS OFF RUST, YOU MEMORY-SAFE PIECE OF GARBAGE
}

struct DumbWriter {
    silence: bool,
    out: std::io::Stdout,
}

impl DumbWriter {
    fn dumb_write(&mut self, s: &impl std::fmt::Display) {
        if !self.silence {
            writeln!(self.out, "{}", s).expect("you're adopted, rust.");
        }
    }

    fn write(&mut self, s: &impl std::fmt::Display) {
        writeln!(self.out, "{}", s).expect("you're adopted, rust.");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Cli = Cli::from_args();
    let run_options = args.run_options.unwrap_or_else(|| RunOptions::None);

    path_test(&args.code)?;

    let mut writer = DumbWriter { silence: args.silence, out: std::io::stdout() };
    if args.gen.is_some() {
        let gen_code = args.gen.unwrap();
        let default = if args.gen_forever { u32::MAX } else { 50 };
        let mut found_bad = false;
        for t in 1..=args.gen_amt.unwrap_or(default) {
            let tc = get_output(
                &gen_code, "",
                &RunOptions::None, t > 1,
                &None, &None,
            )?.0.stdout;  // discard stderr

            let correct = get_output(
                &args.ans.as_ref().unwrap(), &tc,
                &run_options, t > 1,
                &args.prog_fin, &args.prog_fout,
            )?.0.stdout;

            writer.dumb_write(&format!("TEST CASE {}", t).cyan().bold());
            let (normal, file) = get_output(
                &args.code, &tc,
                &run_options, t > 1,
                &args.prog_fin, &args.prog_fout,
            )?;
            prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());
            writer.dumb_write(&format!("exec time: {} s", normal.time).cyan());

            let ans = match args.prog_fout {
                None => normal.stdout,
                Some(_) => file
            };
            let diff_res = validate(
                &ans, &Some(correct), &None,
                t > 1,
                args.whitespace_matters, args.str_case, args.one_abort,
                &mut std::io::stdout(),
            ).with_context(|| "checking error")?;
            if diff_res {
                println!("{}\n{}", "test case failed:".red(), tc.red());
                found_bad = true;
                break;
            }

            writer.dumb_write(&"hooray, test case correct!".bright_green());
        }
        if !found_bad {
            writer.write(&"all correct! (could be good or bad, it depends.)".yellow());
        }
        return Ok(());
    }

    let args_fin = args.fin.with_context(
        || format!("input file or directory not found")
    ).unwrap();

    if args_fin.is_file() {
        let (normal, file) = get_output(
            &args.code, &check_content(&args_fin).unwrap(),
            &run_options, false,
            &args.prog_fin, &args.prog_fout,
        )?;

        prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());
        writer.dumb_write(&format!("exec time: {} s", normal.time).cyan());

        let ans = match args.prog_fout {
            None => normal.stdout,
            Some(_) => file
        };
        let diff_res = validate(
            &ans,
            &if let Some(f) = args.fout { Some(check_content(&f)?) } else { None },
            &args.checker,
            false, args.whitespace_matters, args.str_case, args.one_abort,
            &mut std::io::stdout(),
        ).with_context(|| "checking error")?;
        if !diff_res {
            writer.dumb_write(&"hooray, test case correct!".bright_green());
        }
    } else {
        let default = "{}.in".to_string();
        let fin_fmt = args.fin_fmt.as_ref().unwrap_or(&default);
        let default = "{}.out".to_string();
        let fout_fmt = args.fout_fmt.as_ref().unwrap_or(&default);

        let once = fin_fmt.matches(FMT_TOKEN).count() == 0
            && fout_fmt.matches(FMT_TOKEN).count() == 0;

        let mut t = 1;
        let mut found_bad = true;
        loop {
            let fin_name = fin_fmt.replace(FMT_TOKEN, &t.to_string());

            let mut fin = args_fin.clone();
            fin.extend(&[fin_name]);
            if !fin.is_file() {
                eprintln!("{:?} doesn't exist, stopping testing loop", fin);
                break;
            }

            writer.dumb_write(&format!("TEST CASE {}", t).cyan().bold());
            let (normal, file) = get_output(
                &args.code, &check_content(&fin)?,
                &run_options, t > 1,
                &args.prog_fin, &args.prog_fout,
            )?;
            prog_res(&normal, args.prog_stdout, args.prog_stderr, &mut std::io::stdout());
            writer.dumb_write(&format!("exec time: {} s", normal.time).cyan());

            let mut fout = None;
            if let Some(f) = &args.fout {
                let fout_name = fout_fmt.replace(FMT_TOKEN, &t.to_string());
                let mut fout_path = f.clone();
                fout_path.extend(&[fout_name]);
                fout = Some(check_content(&fout_path)?);
            }

            let ans = match args.prog_fout {
                None => normal.stdout,
                Some(_) => file
            };
            let correct = validate(
                &ans, &fout, &args.checker,
                t > 1,
                args.whitespace_matters, args.str_case, args.one_abort,
                &mut std::io::stdout(),
            ).with_context(|| "checking error")?;
            if correct {
                writer.dumb_write(&"hooray, test case correct!".bright_green());
            } else {
                found_bad = true;
            }

            t += 1;
            if once {
                break;
            }
        }
        if !found_bad {
            writer.write(&"all correct! (could be good or bad, it depends.)".yellow());
        }
    }

    Ok(())
}
