use std::path::{PathBuf};
use std::ffi::OsStr;
use std::io::ErrorKind;
use std::process::Command;

use strum::{IntoEnumIterator};
use strum_macros::{EnumIter, IntoStaticStr};

use crate::errors::*;

#[derive(Debug, Copy, Clone, EnumIter, IntoStaticStr)]
pub enum Lang {
    Python, Java, Cpp
}

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

        match lang.unwrap() {
            Lang::Python => {
                let py_res = Command::new("python").arg(path_str(code)).spawn();
                match py_res {
                    Ok(_) => {}
                    Err(e) => {
                        if let ErrorKind::NotFound = e.kind() {
                            println!("python wasn't found")
                        } else {
                            println!("what");
                        }
                    }
                }
            }
            Lang::Java => {

            }
            Lang::Cpp => {
                let gcc_res = Command::new("g++").arg(path_str(code)).spawn();
                match gcc_res {
                    Ok(_) => {}
                    Err(e) => {
                        if let ErrorKind::NotFound = e.kind() {
                            println!("g++ wasn't found")
                        } else {
                            println!("what");
                        }
                    }
                }
            }
        };
        Ok(())
    }
}

pub fn path_ext(path: &PathBuf) -> Option<&str> {
    path.extension().and_then(OsStr::to_str)
}

pub fn path_str(path: &PathBuf) -> String {
    path.clone().into_os_string().into_string().unwrap()
}

pub fn file_lang(file: &PathBuf) -> Option<Lang> {
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

pub fn check_content(file: &PathBuf) -> Result<String, PathNotFound> {
    if file.is_file() {
        return Ok(std::fs::read_to_string(file).unwrap());
    }
    Err(PathNotFound { path: file.clone() })
}
