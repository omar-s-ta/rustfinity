use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    if numbers.len().is_multiple_of(2) {
        (numbers[numbers.len() / 2] + numbers[numbers.len() / 2 - 1]) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut freq = HashMap::new();
    numbers.iter().for_each(|n| {
        freq.entry(*n).and_modify(|v| *v += 1).or_insert(0);
    });

    let big = *freq.values().max().unwrap();
    let mut res = freq
        .into_iter()
        .filter(|(_, v)| *v == big)
        .map(|(k, _)| k)
        .collect::<Vec<_>>();

    res.sort();
    res
}

pub fn _mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for n in numbers {
        if let Some(value) = map.get_mut(n) {
            *value += 1;
        } else {
            map.insert(*n, 0);
        }
    }
    if let Some(big) = map.values().max() {
        let mut res = Vec::new();
        for (k, v) in map.iter() {
            if *v == *big {
                res.push(*k);
            }
        }
        res.sort();
        res
    } else {
        vec![]
    }
}
