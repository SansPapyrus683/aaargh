use std::fmt::Display;
use colored::Colorize;
use regex::Regex;

fn writeln(s: &(impl Display + ?Sized), mut out: impl std::io::Write) {
    if let Err(e) = writeln!(out, "{}", s) {
        eprintln!("wtf why can't i write: {}", e);
    }
}

pub fn diff_lines<'a>(
    given: impl IntoIterator<Item = &'a str>,
    actual: impl IntoIterator<Item = &'a str>,
    whitespace_matters: bool,
    mut out: impl std::io::Write
) {
    let mut g_iter = given.into_iter();
    let mut a_iter = actual.into_iter();

    let mut line_num = 0;
    loop {
        line_num += 1;
        let g_line: Option<&str> = g_iter.next();
        let a_line: Option<&str> = a_iter.next();
        match (g_line, a_line) {
            (None, None) => break,
            (Some(g), Some(a)) => {
                if g == a {
                    continue;
                }

                let go = Output::parse(g);
                let ao = Output::parse(a);
                if go == ao && !whitespace_matters {
                    continue;
                }

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
                        "read values seem to be the same, but there seems to be an error in formatting\n",
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
            }
            (g, _) => {
                let thing = if g.is_none() {
                    ("actual", "given")
                } else { ("given", "actual") };

                let tp = format!(
                    "{} file has more lines than the {} file", thing.0, thing.1
                ).red();
                writeln(&tp, &mut out);
                break;
            }
        }
    }
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
    fn parse(s: &str) -> Self {
        let s = s.trim();
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
