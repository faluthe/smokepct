use std::env;
use std::{fs::File, io::Write, thread, time::Instant};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};

mod utilities;

use utilities::{permute, factorial, dump_manifest};
use utilities::knowns::{populate_knowns, remove_knowns, restore_knowns, run_stride};
use utilities::unit_tests::dry_run;
use utilities::args::{self, Opts};

// Options
// . DEBUG = {0, 1, 2, 3, 4, 5} (level of verbosity)
const USE_CMD: bool = true;
const DEBUG: usize = 1;
const PRINT: bool = true;
const LOGS: bool = true;
const BENCH: bool = false;

fn smoke_pct(pre_knowns: &str, arguments: &Opts) {
    let thread_count = arguments.thread_count;
    let letters = arguments.letters.clone();
    let pct_x = arguments.pct_x.clone();
    let max_permutations = factorial(letters.len());

    let start = Instant::now();
    let mut threads = vec![];

    if PRINT == true {
        println!("[ pct{} :: {} ]", pct_x, letters);
        println!("base: {}", letters.chars().count());
        println!("max: {} \n", max_permutations.to_formatted_string(&Locale::en));
    }

    for t in 0..thread_count {
        let sums = dump_manifest(String::from("MANIFEST/".to_owned() + &pct_x));
        let b2b = Blake2bSum::new(64);

        let mut tmp_key = letters.clone();
        let known_values = populate_knowns(Some(pre_knowns));
        remove_knowns(&mut tmp_key, known_values.to_owned());
                
        let new_max = factorial(tmp_key.len());
        let block = &new_max / thread_count;
        let max = block + (block * t);
        let min = max - block;
        
        if PRINT == true {
            println!("thread: ({}) {} as base {}: {}", 
                    t, letters, tmp_key.chars().count(), tmp_key);
            println!("\t[min: {:>16}\tmax: {:>16}]", 
                    min.to_formatted_string(&Locale::en), 
                    max.to_formatted_string(&Locale::en));
            println!("\t[old: {:>16}\tnew: {:>16}]", permute(min, tmp_key.chars().collect()), permute(max, tmp_key.chars().collect()));
        }

        let known_values_cp = known_values.clone();
        let thread_block = thread::spawn(move || { 
            for k in min..max {
                let mut x = permute(k, tmp_key.chars().collect());
                restore_knowns(&mut x, &known_values_cp);
                let check = b2b.read_str(x.clone() + "\n");

                if DEBUG > 4 {
                    println!("x: {}", x);
                }

                if sums.contains(&check) {
                    if PRINT == true {
                        println!("Found solution: {} [took {}ms]", 
                            x, start.elapsed().as_millis());
                    }
                    break;
                }
            }
        });
        threads.push(thread_block);
    }
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    if PRINT == true {
        println!("\n . . \n");
        println!(" [ pct{} :: {} ]", pct_x, letters);
        println!("  . puzzle_base: {}", letters.chars().count());
        println!("  . iters: {}", max_permutations.to_formatted_string(&Locale::en));
        println!("  . threads: {}", thread_count);
        println!("  . time: {}ms", start.elapsed().as_millis());
    }

    if LOGS == true {
        let mut log_file = File::options().append(true).create(true).open(
                "logs/timers_".to_owned() + &pct_x + ".log")
                .expect("Error creating log_file");
    
        log_file.write(format!(
                "{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", 
                "Key:", letters, 
                "Threads:", thread_count, 
                "Base:", letters.chars().count(), 
                "Time:", start.elapsed().as_millis() 
            ).as_bytes()).unwrap();
    }
}

// Unit_test Constants
const THREADS: usize = 8;
const PZL_KEY: &str = "EFNOPQRSTUVWXY";
const KNOWNS: &str =  "T____________X";
const STRIDE: &str = "SNF";

fn main() {
    if USE_CMD == true {
        // Get arguments
        let arguments = args::get_options(env::args());
        smoke_pct(KNOWNS, &arguments);
        
        if BENCH == true {
            dry_run(arguments.letters.chars().count(), arguments.thread_count);
        }




        
    } else {
        // Run unit_tests
        const MAN_FILE: &str = "F";
        let const_opts = Opts{ 
            thread_count: THREADS, 
            letters: PZL_KEY.to_string(), 
            pct_x: MAN_FILE.to_string(), 
            known_letters: None, 
            verbosity: DEBUG};

        smoke_pct(KNOWNS, Some(&const_opts).unwrap());

        println!("{}", PZL_KEY);

        let some_vec = run_stride(PZL_KEY, STRIDE, KNOWNS);
        println!("KEY END {}", PZL_KEY);
        println!("{:?}", some_vec);
        for v in some_vec {
            smoke_pct(&v, Some(&const_opts).unwrap());
        }
    }  
}