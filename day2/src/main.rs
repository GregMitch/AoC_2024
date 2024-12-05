use std::fs;

fn main() {
    let file_path = "./input.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines: Vec<&str> = contents.lines().collect();

    let mut safe_reports = 0;
    for line in &lines {
        let report: Vec<&str> = line.split(" ").collect();
        let mut is_inc = true;
        let mut is_dec = true;
        let mut has_gap = false;

        for i in 0..report.len() - 1 {
            let cur_value = report[i].parse::<i32>().unwrap();
            let next_value = report[i+1].parse::<i32>().unwrap();
            
            if cur_value < next_value {
                is_dec = false;
            }

            if cur_value > next_value {
                is_inc = false;
            }
            
            if (next_value - cur_value).abs() > 3 || (next_value - cur_value).abs() < 1 {
                has_gap = true;
            }
        }

        if (is_dec || is_inc) && !has_gap {
            safe_reports += 1;
        }
    }

    println!("{safe_reports}");
}