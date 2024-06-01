use std::env;

use overlapping_intervals::erase_overlap_intervals;

mod daily_temperatures;
mod overlapping_intervals;


fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let v:Vec<Vec<i32>> = vec![
        vec![1,100],
        vec![11,22],
        vec![1,11],
        vec![2,12]
    ];
    let s = format!("{:?}", erase_overlap_intervals(v));

    println!("{}", s);
    println!("Hello, world!");
}
