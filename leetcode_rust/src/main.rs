use std::env;

use overlapping_intervals::find_min_arrow_shots;

mod daily_temperatures;
mod overlapping_intervals;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let v:Vec<Vec<i32>> = vec![
        vec![1,2],
        vec![3,4],
        vec![5,6],
        vec![7,8]
    ];
    let s = format!("{:?}", find_min_arrow_shots(v));

    println!("{}", s);
    println!("Hello, world!");
}
