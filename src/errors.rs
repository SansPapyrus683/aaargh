use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::fcheck;

#[derive(Debug, Clone)]
pub struct PathNotFound { pub path: std::path::PathBuf }

impl Error for PathNotFound {  }

impl Display for PathNotFound {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "the path {} doesn't exist", crate::fcheck::path_str(&self.path))
    }
}

#[derive(Debug, Clone)]
pub struct BadLangError { pub ext: String }

impl Error for BadLangError {  }

impl Display for BadLangError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "the language {} doesn't exist", self.ext)
    }
}

#[derive(Debug, Clone)]
pub struct LangNotFoundError { pub lang: crate::fcheck::Lang }

impl Error for LangNotFoundError {  }

impl Display for LangNotFoundError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let name: &'static str = self.lang.into();  // cursed
        write!(f, "{} not found on the system path", name)
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError { pub err: String }

impl Error for RuntimeError {  }

impl Display for RuntimeError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error while running script:\n{}", self.err)
    }
}

#[derive(Debug, Clone)]
pub enum ExecError {
    PathNotFound(PathNotFound),
    BadLang(BadLangError),
    LangNotFound(LangNotFoundError),
    RuntimeError(RuntimeError)
}

impl ExecError {
    pub fn lang_not_found(lang: fcheck::Lang) -> ExecError {
        return Self::LangNotFound(LangNotFoundError { lang });
    }
}

impl Error for ExecError { }

impl Display for ExecError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ExecError::BadLang(e) => Display::fmt(e, f),
            ExecError::LangNotFound(e) => Display::fmt(e, f),
            ExecError::PathNotFound(e) => Display::fmt(e, f),
            ExecError::RuntimeError(e) => Display::fmt(e, f)
        }
    }
}
