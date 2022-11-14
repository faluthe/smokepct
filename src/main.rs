use std::{fs::File, io::Write, thread, time::Instant};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};

mod utilities;

use utilities::{permute, factorial, dump_manifest};
use utilities::knowns::{populate_knowns, remove_knowns, restore_knowns};
use utilities::unit_tests::benchmarks;

// Initial Data
const THREADS: usize = 16;
const PZL_KEY: &str = "";
const KNOWNS: &str =  "______________Q";
const MAN_FILE: &str = "D";

fn smoke_pct() {
    let max_permutations = factorial(PZL_KEY.len());
    let start = Instant::now();
    let mut threads = vec![];

    println!("[ pct{} :: {} ]", MAN_FILE, PZL_KEY);
    println!("base: {}", PZL_KEY.chars().count());
    println!("max: {} \n", max_permutations.to_formatted_string(&Locale::en));

    for t in 0..THREADS {
        let sums = dump_manifest(MAN_FILE);
        let b2b = Blake2bSum::new(64);

        let mut tmp_key = String::from(PZL_KEY);
        let known_values = populate_knowns(KNOWNS);
        remove_knowns(&mut tmp_key, known_values.to_owned());
                
        let new_max = factorial(tmp_key.len());
        let block = &new_max / THREADS;
        let max = block + (block * t);
        let min = max - block;
        
        println!("thread: ({}) {} as base {}: {}", 
                t, PZL_KEY, tmp_key.chars().count(), tmp_key);
        println!("\t[min: {:>16}\tmax: {:>16}]", 
                min.to_formatted_string(&Locale::en), 
                max.to_formatted_string(&Locale::en));
                
        let known_values_cp = known_values.clone();
        let thread_block = thread::spawn(move || { 
            for k in min..max {
                let mut x = permute(k, tmp_key.chars().collect());
                restore_knowns(&mut x, &known_values_cp);
                let check = b2b.read_str(x.clone() + "\n");

                // println!("x: {}", x);

                if sums.contains(&check) {
                    println!("Found solution: {} [took {}ms]", x, start.elapsed().as_millis());
                    break;
                }
            }
        });
        threads.push(thread_block);
    }
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    println!("\n . . \n");
    println!(" [ pct{} :: {} ]", MAN_FILE, PZL_KEY);
    println!("  . puzzle_base: {}", PZL_KEY.chars().count());
    println!("  . iters: {}", max_permutations.to_formatted_string(&Locale::en));
    println!("  . threads: {}", THREADS);
    println!("  . time: {}ms", start.elapsed().as_millis());

    let mut log_file = File::options().append(true).create(true).open(
            "logs/timers_".to_owned() + MAN_FILE + ".log")
            .expect("Error creating log_file");

    log_file.write(format!(
            "{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", 
            "Key:", PZL_KEY, 
            "Threads:", THREADS, 
            "Base:", PZL_KEY.chars().count(), 
            "Time:", start.elapsed().as_millis() 
        ).as_bytes()).unwrap();
}

fn main() {
    smoke_pct();
    // benchmarks(11, THREADS);

}
