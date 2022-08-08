use colored::Colorize;
use regex::Regex;

pub fn diff_lines<'a>(
    given: impl IntoIterator<Item = &'a str>,
    actual: impl IntoIterator<Item = &'a str>,
    mut out: impl std::io::Write
) {
    let mut g_iter = given.into_iter();
    let mut a_iter = actual.into_iter();

    loop {
        let g_line = g_iter.next();
        let a_line = a_iter.next();
        match (g_line, a_line) {
            (None, None) => break,
            (Some(_), None) => {
                writeln!(out, "{}", "btw your output has more lines than the actual output, just sayin'".red());
                break;
            }
            (None, Some(_)) => {
                writeln!(out, "{}", "btw your output has less lines than the actual output, just sayin'".red());
                break;
            }
            (Some(g), Some(a)) => {
                if g != a {
                    let to_print = format!("'{}' doesn't match '{}'", g, a).red();
                    writeln!(out, "{}", to_print);
                }
            }
        }
    }
}
