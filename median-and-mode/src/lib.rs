use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();

    let length = numbers.len();
    if length % 2 == 0 {
        return ((numbers[length / 2 - 1] + numbers[length / 2]) as f32) / 2 as f32;
    } else {
        return numbers[length / 2] as f32;
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut occurrences = HashMap::new();

    for &num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }

    let max_count = occurrences.values().cloned().max().unwrap_or(0);

    let mut modes: Vec<i32> = occurrences
        .into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(num, _)| num)
        .collect();

    modes.sort();

    modes
}
