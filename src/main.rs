use once_cell_regex::regex;

fn main() {
    let re = regex!("hello");
    let matched = re.is_match("hello world");
    println!("{}", matched);
}
