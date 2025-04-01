use regex::Regex;

pub fn find_headers(content: &str) {
    let finder = Regex::new(r"\n\n([\d.]*)\s*([^\n]+)\n\n").unwrap();
    for res in finder.find_iter(content) {
        println!("header match: {}", res.as_str().trim())
    }
}