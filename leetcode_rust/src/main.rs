use std::env;

use daily_temperatures::daily_temperatures;

mod daily_temperatures;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let v = vec![34,80,80,34,34,80,80,80,80,34];
    let s = format!("{:?}", daily_temperatures(v));

    println!("{}", s);
    println!("Hello, world!");
}
