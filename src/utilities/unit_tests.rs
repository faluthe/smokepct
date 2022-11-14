use std::{fs::File, io::{Write}, thread, time::{Instant}};
use num_format::{ToFormattedString, Locale};

use crate::{permute, factorial};
// fn permute(mut k: usize, mut string: Vec<char>) -> String {
//     for i in 1..string.len() {
//         string.swap(k % (i + 1), i);
//         k = k / (i + 1);
//     }
//     string.into_iter().collect()
// }

// fn factorial(x: usize) -> usize {
//     if x == 1 {
//         x
//     } else {
//         x * factorial(x - 1)
//     }
// }


pub fn benchmarks(base_height: usize, thread_count: usize) {
    // Benchmark the permutations of threaded Base1 -> BaseN
    let mut log_file = File::options().append(true).create(true).open(
        "logs/".to_owned() + "BENCHMARK" + ".log")
        .expect("Error creating [BENCHMARK.log]");
        
    let mut tmp_key = String::with_capacity(15);
    let mut bank = String::from("FEDCBA");
    let mut i = 0;
    tmp_key.push_str(&i.to_string());

    while tmp_key.len() < base_height {
        let start = Instant::now();
        let max = factorial(tmp_key.len());
        let mut threads = vec![];

        println!("Bank: {} :: Base: {} :: Threads: {}", 
                tmp_key, 
                tmp_key.chars().count(), 
                thread_count);

        for t in 0..thread_count {
            let tmp_key_b = tmp_key.clone();
            
            let block = &max / thread_count;
            let max = block + (block * t);
            let min = max - block;
            
            let thread_block = thread::spawn(move || { 
                for k in min..max {
                    permute(k, tmp_key_b.chars().collect());

                }
            });
            threads.push(thread_block);
        }
        for thread in threads {
            let _ = thread.join().unwrap();
        }

        log_file.write( format!("{:9} '{}'\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", 
                "Key:", tmp_key, 
                "Base:", tmp_key.chars().count(), 
                "Max:", max.to_formatted_string(&Locale::en), 
                "Time:", start.elapsed().as_millis()).as_bytes() ).unwrap();

        if i < 9 {
            i = i + 1;
            tmp_key.push_str(&i.to_string());
        }
        else {
            tmp_key.push_str(&bank.pop().unwrap().to_string());
        }

    }

    log_file.write( format!("\n==================================================================\n").as_bytes() ).unwrap();
}