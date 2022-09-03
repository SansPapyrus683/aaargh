use std::path::{PathBuf};
use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};

use strum::IntoEnumIterator;
use strum_macros::{EnumIter, IntoStaticStr};
use is_executable::IsExecutable;

use crate::RunOptions;
use crate::errors::*;

#[derive(Debug, Copy, Clone, EnumIter, IntoStaticStr)]
pub(crate) enum Lang { Python, Java, Cpp }

impl Lang {
    fn valid_ext(&self) -> Vec<&str> {
        match self {
            Lang::Python => vec!["py"],
            Lang::Java => vec!["java"],
            Lang::Cpp => vec!["cpp", "cc", "cxx", "c++"]
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ProgRes {
    pub(crate) stdout: String,
    pub(crate) stderr: String,
    pub(crate) time: f64
}

/// executes some code from a path given input & whatever
/// ### arguments:
/// * code: path with code, only supports python 3, c++, and java
/// * input: optional input to be passed into stdin
/// * options: `RunOptions` from the main file, contains arguments
///            to be passed to the created compiler/interpreter process
/// * compiled: has this been compiled already?
///   * if it's an interpreted language, has no effect
///   * if it's compiled, this will just run the relevant execution command
pub(crate) fn exec(
    code: &PathBuf, input: &str,
    options: &RunOptions, compiled: bool
) -> Result<ProgRes, ExecError> {
    if !code.is_file() {
        return Err(ExecError::path_not_found(code.clone()));
    }

    let is_exe = code.is_executable();
    let lang = file_lang(code);
    if lang.is_none() && !is_exe {
        let ext = path_ext(code).unwrap_or("");
        return Err(ExecError::bad_lang(ext));
    }

    let options = match options {
        RunOptions::Some(a) => a.clone(),
        RunOptions::None => Vec::new()
    };

    // note to self: https://doc.rust-lang.org/std/time/struct.Instant.html
    let file = path_str(code);
    let mut cmd;
    if is_exe {
        cmd = Command::new(format!("./{}", code.to_str().unwrap()));
    } else {
        match lang.unwrap() {
            Lang::Python => {
                let cmds = vec!["py", "python", "python3"];
                let cmd_use = cmds.iter()
                    .find(|c| cmd_exists(c))
                    .ok_or(ExecError::lang_not_found(Lang::Python))?;

                cmd = Command::new(cmd_use);
                cmd.arg(&file).args(&options);
            }
            Lang::Java => {
                let runner = "java";
                if !compiled {
                    let compiler = "javac";
                    if !cmd_exists(runner) || !cmd_exists(compiler) {
                        return Err(ExecError::lang_not_found(Lang::Java));
                    }
                    let compile_res = Command::new(compiler)
                        .arg(&file)
                        .args(&options)
                        .spawn().expect("JAVA OH NO")
                        // make sure compilation finishes first
                        .wait().expect("bruh...");
                    if !compile_res.success() {
                        return Err(ExecError::runtime_error("java compilation error"));
                    }
                }

                cmd = Command::new(runner);
                cmd.arg(&file);
            }
            Lang::Cpp => {
                let name = code.file_stem().unwrap().to_str().unwrap().trim();
                if !compiled {
                    let compiler = "g++";
                    if !cmd_exists(compiler) {
                        return Err(ExecError::lang_not_found(Lang::Cpp));
                    }
                    let compile_res = Command::new(compiler)
                        .arg(&file)
                        .arg("-o").arg(name)
                        .args(&options)
                        .spawn().expect("C++ OH NO")
                        .wait().expect("bruh...");
                    if !compile_res.success() {
                        return Err(ExecError::runtime_error("cpp compilation error"));
                    }
                }

                cmd = Command::new(format!("./{}", name));
            }
        };
    }

    let mut cmd = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("something terribly wrong has happened");

    let start = std::time::Instant::now();

    let mut writer = std::io::BufWriter::new(cmd.stdin.take().unwrap());
    // https://stackoverflow.com/questions/21615188
    for l in input.lines() {
        let eol = '\n';
        let mut l = l.to_string();
        l.push(eol);
        writer.write_all(l.as_bytes()).expect("INPUT OH NO");
    }
    writer.flush().expect("god i'm so tired");

    let output = cmd.wait_with_output().expect("bruh...");
    let time = start.elapsed();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        return Err(ExecError::runtime_error(&stderr));
    }
    Ok(ProgRes { stdout, stderr, time: time.as_secs_f64() })
}

fn file_lang(file: &PathBuf) -> Option<Lang> {
    let ext = path_ext(file);
    if ext.is_none() {
        return None;
    }
    let ext = ext.unwrap();
    for l in Lang::iter() {
        if l.valid_ext().contains(&ext) {
            return Some(l);
        }
    }
    None
}

fn cmd_exists(cmd: &str) -> bool {
    match Command::new(cmd)
        .arg("--version")
        .stdout(Stdio::piped()).spawn() {
        Ok(_) => true,
        Err(e) => e.kind() == ErrorKind::NotFound
    }
}

// general utility methods
pub(crate) fn path_ext(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(std::ffi::OsStr::to_str)
}

pub(crate) fn path_str(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub(crate) fn check_content(file: &PathBuf) -> Result<String, PathNotFound> {
    if file.is_file() {
        return Ok(std::fs::read_to_string(file).unwrap());
    }
    Err(PathNotFound { path: file.clone() })
}
