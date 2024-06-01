use std::i32::MIN;

#[allow(dead_code)]
pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
    let mut ints = intervals.clone();
    ints.sort_by(|a, b| a[1].cmp(&b[1]));
    
    let mut count = 0;
    let mut min:i32 = MIN;
    for vec in ints {
        if vec[0] >= min {
            min = vec[1];
            continue;
        }

        count += 1;
    }

    count
}