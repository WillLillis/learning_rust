use std::collections::HashMap;

fn main() {
    let nums = vec![1, 2, 3, 4, 4, 4];
    
    println!("The median is {}.", median(&nums));
    println!("The mode is {}.", mode(&nums));
}


fn median(arr: &Vec<i32>) -> f64 {
    let mut new_vec = arr.to_vec();
    new_vec.sort();

    return match arr.len() % 2 {
        0 => (arr[(arr.len() / 2) - 1] as f64 + arr[arr.len() / 2] as f64) / 2.0, // there isn't a middle element, take the average 
        1 => arr[arr.len() / 2] as f64, // there is a middle element, return it
        _ => unreachable!(),
    };
}

fn mode(arr: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &num in arr {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }
    
    let mut max_key = 0;
    let mut max_val = 0;

    for (key, value) in &counts {
        if *value > max_val {
            max_key = *key;
            max_val = *value;
        }
    }

    return max_key;
}
