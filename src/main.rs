use std::{fs::File, io::Write, thread, time::Instant};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};

mod utilities;

use utilities::{permute, factorial, dump_manifest};
use utilities::knowns::{populate_knowns, remove_knowns, restore_knowns, generate_knowns};
use utilities::unit_tests::dry_run;

// Options
// . DEBUG = {0, 1, 2, 3, 4, 5} (level of verbosity)
const DEBUG: usize = 0;
const PRINT: bool = true;
const LOGS: bool = true;
const BENCH: bool = false;

// Initial Data
const THREADS: usize = 8;
const PZL_KEY: &str = "FHJLMNOPQRSTUWY";
const KNOWNS: &str =  "Q______________";
// const KNOWNS: &str =  "ABCD____________";

const MAN_FILE: &str = "D";

fn smoke_pct(pre_knowns: &str) {
    let max_permutations = factorial(PZL_KEY.len());
    let start = Instant::now();
    let mut threads = vec![];

    if PRINT == true {
        println!("[ pct{} :: {} ]", MAN_FILE, PZL_KEY);
        println!("base: {}", PZL_KEY.chars().count());
        println!("max: {} \n", max_permutations.to_formatted_string(&Locale::en));
    }

    for t in 0..THREADS {
        let sums = dump_manifest(MAN_FILE);
        let b2b = Blake2bSum::new(64);

        let mut tmp_key = String::from(PZL_KEY);
        let known_values = populate_knowns(Some(pre_knowns));
        remove_knowns(&mut tmp_key, known_values.to_owned());
                
        let new_max = factorial(tmp_key.len());
        let block = &new_max / THREADS;
        let max = block + (block * t);
        let min = max - block;
        
        if PRINT == true {
            println!("thread: ({}) {} as base {}: {}", 
                    t, PZL_KEY, tmp_key.chars().count(), tmp_key);
            println!("\t[min: {:>16}\tmax: {:>16}]", 
                    min.to_formatted_string(&Locale::en), 
                    max.to_formatted_string(&Locale::en));
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
        println!(" [ pct{} :: {} ]", MAN_FILE, PZL_KEY);
        println!("  . puzzle_base: {}", PZL_KEY.chars().count());
        println!("  . iters: {}", max_permutations.to_formatted_string(&Locale::en));
        println!("  . threads: {}", THREADS);
        println!("  . time: {}ms", start.elapsed().as_millis());
    }

    if LOGS == true {
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
}

fn main() {
    const ALL_KNOWNS: [&str; 12] = [
    "QMHL___________",
    "Q_MHL__________",
    "Q__MHL_________",
    "Q___MHL________",
    "Q____MHL_______",
    "Q_____MHL______",
    "Q______MHL_____",
    "Q_______MHL____",
    "Q________MHL___",
    "Q_________MHL__",
    "Q__________MHL_",
    "Q___________MHL",
    ];
    let mut i = 0;
    for k in ALL_KNOWNS {
        println!("RUNNING ITER: {} on {}", i, k);
        smoke_pct(k);
        i = i + 1;
    }

    let tmpvec = populate_knowns(Some(KNOWNS));
    generate_knowns(PZL_KEY, Some(&tmpvec));

    populate_knowns(None);
    generate_knowns(PZL_KEY, None);

    if BENCH == true {
        dry_run(PZL_KEY.chars().count(), THREADS);
    }

}
