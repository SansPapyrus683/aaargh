use std::path::{PathBuf};
use std::ffi::{OsStr, OsString};
use std::io::{ErrorKind, Write};
use std::process::{Command, Stdio};

use strum::{IntoEnumIterator};
use strum_macros::{EnumIter, IntoStaticStr};

use crate::RunOptions;
use crate::errors::*;

#[derive(Debug, Copy, Clone, EnumIter, IntoStaticStr)]
pub enum Lang { Python, Java, Cpp }

impl Lang {
    fn valid_ext(&self) -> Vec<&str> {
        match self {
            Lang::Python => vec!["py"],
            Lang::Java => vec!["java"],
            Lang::Cpp => vec!["cpp", "cc", "cxx", "c++"]
        }
    }
}

pub fn exec(
    code: &PathBuf, input: Option<&str>, args: &RunOptions,
) -> Result<(String, String), ExecError> {
    match check_content(code) {
        Ok(_) => {}
        Err(e) => return Err(ExecError::PathNotFound(e))
    };

    let lang = file_lang(code);
    if lang.is_none() {
        let ext = path_ext(code).unwrap_or("");
        return Err(ExecError::BadLang(BadLangError { ext: ext.to_string() }));
    }

    let options = match args {
        RunOptions::Some(a) => a.clone(),
        RunOptions::None => Vec::new()
    };

    let file = path_str(code);
    let mut cmd;
    match lang.unwrap() {
        Lang::Python => {
            let cmds = vec!["py", "python", "python3"];
            let cmd_use = cmds.iter().find(|c| cmd_exists(c))
                .ok_or(ExecError::lang_not_found(Lang::Python))?;

            cmd = Command::new(cmd_use);
            cmd.arg(&file).args(&options);
        }
        Lang::Java => {
            let compiler = "javac";
            let runner = "java";
            if !cmd_exists(compiler) || !cmd_exists(compiler) {
                return Err(ExecError::lang_not_found(Lang::Java));
            }
            Command::new(compiler)
                .arg(&file)
                .args(&options)
                .spawn().expect("JAVA OH NO");

            cmd = Command::new(runner);
            cmd.arg(&file);
        }
        Lang::Cpp => {
            let compiler = "g++";
            if !cmd_exists(compiler) {
                return Err(ExecError::lang_not_found(Lang::Cpp));
            }
            Command::new(compiler)
                .arg(&file)
                .args(&options)
                .spawn().expect("C++ OH NO");

            let cmd_name = match std::env::consts::OS {
                "linux" => "./a.out",
                "mac" => "./a.out",
                "windows" => "./a",
                _ => ""
            };

            cmd = Command::new(cmd_name);
        }
    };

    let mut cmd = cmd
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("something terribly wrong has happened");

    if let Some(input) = input {
        let mut writer = std::io::BufWriter::new(cmd.stdin.take().unwrap());
        // https://stackoverflow.com/questions/21615188
        for l in input.lines() {
            let eol = '\n';
            let mut l = l.to_string();
            l.push(eol);
            writer.write_all(l.as_bytes()).expect("INPUT OH NO");
        }
        writer.flush().expect("god i'm so tired");
    }

    let output = cmd.wait_with_output().expect("bruh...");
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    if !output.status.success() {
        return Err(ExecError::runtime_error(&stderr));
    }
    Ok((stdout, stderr))
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
pub fn path_ext(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}

pub fn path_str(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn check_content(file: &PathBuf) -> Result<String, PathNotFound> {
    if file.is_file() {
        return Ok(std::fs::read_to_string(file).unwrap());
    }
    Err(PathNotFound { path: file.clone() })
}
