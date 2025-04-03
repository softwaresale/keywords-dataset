use regex::Regex;

pub fn intro_header_regex_factory() -> Regex {
    Regex::new(r"([\d.iI]+)\s*I[nN][tT][rR][oO][dD][uU][cC][tT][iI][oO][nN] *\n\n").unwrap()
}
