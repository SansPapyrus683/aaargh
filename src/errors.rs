use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};
use crate::exec;

#[derive(Debug, Clone)]
pub struct ArgError { pub err: String }

impl Error for ArgError {  }

impl Display for ArgError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "error with arguments:\n{}", self.err)
    }
}

#[derive(Debug, Clone)]
pub struct PathNotFound { pub path: std::path::PathBuf }

impl Error for PathNotFound {  }

impl Display for PathNotFound {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "the path {} doesn't exist", exec::path_str(&self.path))
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
pub struct LangNotFoundError { pub lang: exec::Lang }

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
    pub fn path_not_found(path: std::path::PathBuf) -> ExecError {
        Self::PathNotFound(PathNotFound { path })
    }

    pub fn bad_lang(ext: &str) -> ExecError {
        Self::BadLang(BadLangError { ext: ext.to_string() })
    }

    pub fn lang_not_found(lang: exec::Lang) -> ExecError {
        Self::LangNotFound(LangNotFoundError { lang })
    }

    pub fn runtime_error(err: &str) -> ExecError {
        Self::RuntimeError(RuntimeError { err: err.to_string() })
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