pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut stack: Vec<usize> = Vec::with_capacity(temperatures.len());
    let mut result: Vec<i32> = vec![0; temperatures.len()];
    
    stack.push(0);
    for i in 1..temperatures.len() {
        let u = stack.last().copied();
        if u.is_none() || temperatures[u.unwrap()] >= temperatures[i] {
            stack.push(i);
            continue;
        }

        while let Some(index) = stack.last().copied() {
            if temperatures[i] <= temperatures[index] {
                break;
            }

            result[index] = (i - index) as i32;
            stack.pop();
        }

        stack.push(i);
    }

    return result;
}