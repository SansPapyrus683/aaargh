use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
pub(crate) struct Cli {
    /// code file (only supports c++, py, and java)
    #[structopt()]
    pub(crate) code: PathBuf,

    /// generator code, only supports stdout because i'm a lazy frick
    #[structopt(long = "gen", short = "g")]
    pub(crate) gen: Option<PathBuf>,

    /// correct code (required only if given generator)
    #[structopt(long = "ans", short = "a", requires("gen"))]
    pub(crate) ans: Option<PathBuf>,

    #[structopt(long = "gen-amt", requires("gen"))]
    pub(crate) test_amt: Option<u32>,

    /// file or directory to use for input
    #[structopt(long = "fin", conflicts_with("gen"), required_unless("gen"))]
    pub(crate) fin: Option<PathBuf>,

    /// file or directory that contains the actual outputs
    #[structopt(long = "fout", conflicts_with("ans"), requires("fin"))]
    pub(crate) fout: Option<PathBuf>,  // no clue why i have to Option<> this

    #[structopt(long = "check", short = "c", conflicts_with("fout"), conflicts_with("ans"))]
    pub(crate) checker: Option<PathBuf>,

    /// note: won't be used if `fin` & `fout` are normal files
    /// the format string for the input files
    /// (occurrences of `{}` will be replaced with numbers starting from 1)
    #[structopt(long = "fin-fmt")]
    pub(crate) fin_fmt: Option<String>,

    /// the format string for the output files (basically same thing as `fin_fmt`)
    #[structopt(long = "fout-fmt")]
    pub(crate) fout_fmt: Option<String>,

    /// file name to use for input (if `None`, stdin will be used)
    #[structopt(long = "prog-fin")]
    pub(crate) prog_fin: Option<PathBuf>,

    /// file name to detect for output (if `None`, stdout will be used)
    #[structopt(long = "prog-fout")]
    pub(crate) prog_fout: Option<PathBuf>,

    // flags for custom grading/running options

    /// some graders don't care how you space your numbers.
    /// if your grader isn't one of these, set this flag
    #[structopt(long = "whitespace-fmt")]
    pub(crate) whitespace_matters: bool,

    /// when comparing strings, should capitalization & the like matter?
    #[structopt(long = "str-case")]
    pub(crate) str_case: bool,

    /// this makes the output checker stop as soon as it detects a discrepancy
    /// (i.e. it won't go any further because it's already wrong)
    #[structopt(long = "one-abort")]
    pub(crate) one_abort: bool,

    /// should the programs output the stdout w/ the diff results?
    #[structopt(long = "prog-stdout")]
    pub(crate) prog_stdout: bool,

    /// should the programs output the stderr w/ diff the results?
    #[structopt(long = "prog-stderr")]
    pub(crate) prog_stderr: bool,

    #[structopt(subcommand)]
    pub(crate) run_options: Option<RunOptions>
}

// https://docs.rs/structopt/latest/structopt/#external-subcommands
#[derive(Debug, PartialEq, StructOpt)]
pub(crate) enum RunOptions {
    None,
    #[structopt(external_subcommand)]
    Some(Vec<std::ffi::OsString>)
}
