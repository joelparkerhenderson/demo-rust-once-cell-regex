# Demo Rust once-cell-regex

Demo of Rust programming language, once-cell-regex crate, and `regex!` macro that creates lazy static regular expressions.

Example:

```rust
use once_cell_regex::regex;

fn main() {
    let re = regex!("hello");
    let matched = re.is_match("hello world");
    println!("{}", b);
}
```

Example with `once_cell` instead of `once_cell_regex`:

```rust
use regex::Regex;
use once_cell::sync::Lazy;

fn main() {
    static RE: Lazy<Regex> = Lazy::new(||Regex::new("hello").unwrap());
    let matched = RE.is_match("hello world");
    println!("{}", matched);
}
```
