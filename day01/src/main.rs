use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    let mut list_a: Vec<i32> = Vec::new();
    let mut list_b: Vec<i32> = Vec::new();
    let mut map_b: HashMap<i32, i32> = HashMap::new();
    while io::stdin().read_line(&mut line).expect("failed to read line") != 0 {
        let pair: Vec<i32> = line.trim().split_whitespace()
            .map(|x| x.parse().expect(&format!("not an integer: {:?}", x)))
            .collect();
        list_a.push(pair[0]);
        list_b.push(pair[1]);
        map_b.entry(pair[1]).and_modify(|count| *count += 1).or_insert(1);
        line = String::new();
    }
    list_a.sort();
    list_b.sort();
    let mut list_c: Vec<i32> = Vec::new();
    for i in 0..list_a.len() {
        let diff: i32 = (list_a[i] - list_b[i]).abs();
        list_c.push(diff);
    }
    let mut sum_b:i32 = 0;
    for i in 0..list_a.len() {
        let value = list_a[i];
        let number = match map_b.get(&value) {
            Some(n) => *n,
            None => 0,
        };
        sum_b += value * number;

    }
    println!("{:?}", list_c.into_iter().sum::<i32>());
    println!("{:?}", sum_b);
}
