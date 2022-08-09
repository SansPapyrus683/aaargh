use std::path::{PathBuf};
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::Command;

use strum::{IntoEnumIterator};
use strum_macros::{EnumIter, IntoStaticStr};

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

    pub fn exec(code: &PathBuf) -> Result<(), ExecError> {
        match check_content(code) {
            Ok(_) => {}
            Err(e) => return Err(ExecError::PathNotFound(e))
        }

        let lang = file_lang(code);
        if lang.is_none() {
            let ext = path_ext(code).unwrap_or("");
            return Err(ExecError::BadLang(BadLangError { ext: ext.to_string() }));
        }

        let file = path_str(code);
        let output = match lang.unwrap() {
            Lang::Python => {
                let cmds = vec!["py", "python", "python3"];
                let cmd_use = cmds.iter().find(|c| cmd_exists(c))
                    .ok_or(ExecError::lang_not_found(Lang::Python))?;
                Command::new(cmd_use).arg(&file).output()
            }
            Lang::Java => {
                let compiler = "javac";
                let runner = "java";
                if !cmd_exists(compiler) || !cmd_exists(compiler) {
                    return Err(ExecError::lang_not_found(Lang::Java));
                }
                Command::new(compiler).arg(&file).spawn().expect("OH NO");
                Command::new(runner).arg(&file).output()
            }
            Lang::Cpp => {
                let compiler = "g++";
                if !cmd_exists(compiler) {
                    return Err(ExecError::lang_not_found(Lang::Cpp));
                }
                Command::new(compiler).arg(&file).spawn().expect("OH NO");
                Command::new("./a").output()
            }
        };
        dbg!(output);
        Ok(())
    }
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
    match Command::new(cmd).arg("--version").spawn() {
        Ok(_) => true,
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                false
            } else {
                true
            }
        }
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
