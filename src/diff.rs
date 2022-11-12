use std::fmt::Display;
use std::iter::zip;
use colored::Colorize;
use regex::Regex;

fn writeln(s: &(impl Display + ?Sized), mut out: impl std::io::Write) {
    if let Err(e) = writeln!(out, "{}", s) {
        eprintln!("wtf why can't i write: {}", e);
    }
}

pub(crate) fn diff_lines<'a>(
    given: impl IntoIterator<Item = &'a str>,
    actual: impl IntoIterator<Item = &'a str>,
    whitespace_matters: bool, str_case: bool, one_abort: bool,
    mut out: impl std::io::Write
) -> bool {
    let mut g_vec: Vec<&str> = given.into_iter().collect();
    let mut a_vec: Vec<&str> = actual.into_iter().collect();

    let mut different = false;

    if !whitespace_matters {
        while let Some(l) = g_vec.last() {
            if !l.trim().is_empty() {
                break;
            }
            g_vec.pop();
        }

        while let Some(l) = a_vec.last() {
            if !l.trim().is_empty() {
                break;
            }
            a_vec.pop();
        }
    }

    if g_vec.len() != a_vec.len() {
        different = true;
        writeln(&format!("{}", "mismatch:".red()), &mut out);
        let thing = if a_vec.len() > g_vec.len() {
            ("answer", "output")
        } else { ("output", "answer") };

        let tp = format!(
            "{} has more lines than the {}", thing.0, thing.1
        ).red();
        writeln(&tp, &mut out);
    }

    let mut line_num = 0;
    for (g, a) in zip(g_vec, a_vec) {
        line_num += 1;
        if g == a {
            continue;
        }

        let mut g = g.to_string();
        let mut a = a.to_string();
        if !str_case {
            g = g.to_lowercase();
            a = a.to_lowercase();
        }

        let go = Output::parse(&g);
        let ao = Output::parse(&a);
        if go == ao && !whitespace_matters {
            continue;
        }

        different = true;
        writeln(&format!(
            "mismatch with {}s at line {line_num}:", go.detected_type()
        ).red(), &mut out);

        if std::mem::discriminant(&go) != std::mem::discriminant(&ao) {
            let tp = format!(
                "output types don't match ({} should be {})",
                go.detected_type(), ao.detected_type()
            );
            writeln(&tp, &mut out);
            continue;
        } else if go == ao {
            let tp = format!(concat!(
                "read values seem to be the same,",
                "but there seems to be an error in formatting\n",
                "given line:\n'{}'\n",
                "actual line:\n'{}'"
            ), g, a);
            writeln(&tp, &mut out);
            continue;
        }

        let diff = match (&go, &ao) {
            (Output::Num(g), Output::Num(a)) =>
                format!("numbers {g} and {a} aren't the same")
            ,
            (Output::NumArr(g), Output::NumArr(a)) => {
                let mut res = "".to_string();
                for d in iter_diff(g, a) {
                    res.push_str(&format!(
                        "numbers at index {} differ ({} should be {})\n",
                        d.pos, d.given, d.actual
                    ));
                }
                res
            },
            (Output::Str(g), Output::Str(a)) => {
                let mut res = "".to_string();
                for d in iter_diff(g.chars(), a.chars()) {
                    res.push_str(&format!(
                        "characters at index {} differ ({} should be {})\n",
                        d.pos, d.given, d.actual
                    ));
                }
                res
            },
            (Output::Other(g), Output::Other(a)) => format!(
                "given line: '{}'\nactual line: '{}'", g, a
            ),
            (_, _) => unreachable!("oh no")
        };
        writeln(&diff, &mut out);
        if one_abort {
            writeln(&"stopping after single diff (one-abort)".red(), &mut out);
            break;
        }
    }
    different
}

#[derive(Debug, PartialEq)]
enum Output {
    Num(f64),
    NumArr(Vec<f64>),
    Str(String),
    Whitespace,
    Other(String)
}

impl Output {
    fn parse<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref().trim();
        if s.is_empty() {
            return Self::Whitespace;
        }

        // https://stackoverflow.com/questions/12643009
        let num_fmt = Regex::new("^([+-]?([0-9]*[.])?[0-9]+)$").unwrap();
        let str_fmt = Regex::new(r"^(\D+)$").unwrap();
        if num_fmt.is_match(s) {
            return Self::Num(s.parse::<f64>().unwrap())
        } else if str_fmt.is_match(s) {
            return Self::Str(s.to_string());
        }

        let arr = s.split_whitespace();
        let all_num = arr.clone().all(|i| num_fmt.is_match(i));
        if all_num {
            return Self::NumArr(arr.map(|n| n.parse::<f64>().unwrap()).collect());
        }

        Self::Other(s.to_string())
    }

    fn detected_type(&self) -> &str {
        match self {
            Output::Num(_) => "number",
            Output::NumArr(_) => "number array",
            Output::Str(_) => "string",
            Output::Whitespace => "whitespace",
            Output::Other(_) => "line"
        }
    }
}

struct Diff<T> { given: T, actual: T, pos: usize }

fn iter_diff<T: PartialEq>(
    given: impl IntoIterator<Item = T>,
    actual: impl IntoIterator<Item = T>
) -> Vec<Diff<T>> {
    let mut res = Vec::new();
    for (i, (g, a)) in given.into_iter().zip(actual.into_iter()).enumerate() {
        if g != a {
            res.push(Diff { given: g, actual: a, pos: i });
        }
    }
    res
}
