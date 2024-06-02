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

#[allow(dead_code)]
pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
    points.sort_by(|a, b| a[1].cmp(&b[1]));

    let mut count = 1;
    let mut min:i32 = points[0][1];
    for i in 1..points.len() {
        if points[i][0] > min {
            min = points[i][1];
            count += 1;
        }
    }

    count
}