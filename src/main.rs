use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

fn read_data(file_path: &str) -> io::Result<(i32, Vec<(i32, i32)>)>{
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let capacity: i32 = lines
    .next()
    .ok_or(io::Error::new(io::ErrorKind::InvalidData, "File is empty!"))??
    .trim()
    .parse::<i32>()
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let items: Vec<(i32, i32)> = lines
        .map(|line| {
            let line: String = line?;
            let parts: Vec<i32> = line
                .split_whitespace()
                .map(|part| part.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            if parts.len() != 2 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Each line must have two numbers",
                ));
            }
            Ok((parts[0], parts[1]))
        }).collect::<Result<Vec<_>, _>>()?;

    Ok((capacity, items))
}

fn evaluate_vector(bitset: u64, items: &[(i32, i32)], capacity: i32) -> (i32, i32) {
    let mut total_weight = 0;
    let mut total_value = 0;
    
    for(i, &(weight, value)) in items.iter().enumerate() {
        if (bitset & (1 << i)) != 0 {
            total_weight += weight;
            if total_weight > capacity {
                return (0, 0)
            }
            total_value += value;
        }
    }

    (total_weight, total_value)
}

fn knapsack_brute_force(file_path: &str) -> io::Result<()> {
    let (capacity, items) = read_data(file_path)?;
    let n = items.len();
    let total_combinations = 1 << n;
    let (best_vector, best_weight, best_value) = (0..total_combinations)
        .into_par_iter()
        .map(|bitset| {
            let (total_weight, total_value) = evaluate_vector(bitset, &items, capacity);
            (bitset, total_weight, total_value)
        })
        .max_by_key(|&(_,_, total_value)| total_value)
        .unwrap();
    
    let best_vector: Vec<u8> = (0..n).map(|i| ((best_vector >> i) & 1) as u8).collect();

    println!("Best characteristic vector: {:?}", best_vector);
    println!("Total weight: {}", best_weight);
    println!("Total value: {}", best_value);
    Ok(())
}

fn main() -> io::Result<()> {

    let start_time = Instant::now();
    knapsack_brute_force("13.txt")?;
    let end_time = Instant::now();
    println!("Execution time: {:.2} seconds", (end_time - start_time).as_secs_f64());
    Ok(())
}