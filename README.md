# Demo Rust once-cell-regex

Demo of Rust programming language, once-cell-regex crate, and `regex!` macro that creates lazy static regular expressions.

Example:

```rust
use once_cell_regex::regex;

fn main() {
    let r = regex!("hello");
    let matched = r.is_match("hello world");
    println!("{}", b);
}
```
