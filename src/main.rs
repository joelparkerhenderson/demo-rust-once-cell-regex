use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let matched = r.is_match("hello world");
    println!("{}", b);
}
