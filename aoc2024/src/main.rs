use std::{collections::HashMap, fs};

fn main() {
    // --snip--
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in lines {
        let points: Vec<&str> = line.split("   ").collect();
        left_list.push(points[0].parse::<i32>().unwrap());
        right_list.push(points[1].parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    //Part 1
    //let distances: Vec<i32> = left_list.into_iter().zip(right_list.into_iter()).map(|(l, r)| (l - r).abs()).collect();
    //let sum: i32 = distances.iter().sum();
    //println!("{sum}");

    let mut hm: HashMap<i32, i32> = HashMap::new();
    let mut score = 0;
    for value in &right_list {
        if hm.contains_key(value) {
            continue;
        }
        let cop = right_list.clone().into_iter().filter(|n| *n == *value).count();
        hm.insert(*value, cop as i32);
    }

    for value in &left_list {
        if let Some(inner) = hm.get(value) {
            score += value * *inner;
        }
        
    }

    println!("{score}")
}
