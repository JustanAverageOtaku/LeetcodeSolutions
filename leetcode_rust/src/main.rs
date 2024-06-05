use std::env;

use trie::suggested_products;

mod daily_temperatures;
mod overlapping_intervals;
pub mod trie;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let v: Vec<String> = vec![
        "mobile".to_string(),
        "mouse".to_string(),
        "moneypot".to_string(),
        "monitor".to_string(),
        "mousepad".to_string()
        ];
    let s = format!("{:?}", suggested_products(v, "mouse".to_string()));

    println!("{}", s);
    println!("Hello, world!");
}
