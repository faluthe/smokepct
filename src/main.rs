use std::{fs::File, io::{Read, Write}, collections::HashSet, thread, time::{Instant}};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};


// Initial Data
const THREADS: usize = 16;
const PZL_KEY: &str = "FHKLMOPQRSUWXY";
const MAN_FILE: &str = "C";

fn dump_manifest() -> HashSet<String> {
    let manifest_path = "MANIFEST/".to_owned() + MAN_FILE;
    let mut manifest = File::open(manifest_path).expect("Manifest not found");
    let mut data = String::new();
    manifest.read_to_string(&mut data).unwrap();
    let mut sums: HashSet<String> = HashSet::new();
    
    for l in data.lines() {
        sums.insert(l.to_uppercase());
    }
    sums
}

fn permute(mut k: usize, mut string: Vec<char>) -> String {
    for i in 1..string.len() {
        string.swap(k % (i + 1), i);
        k = k / (i + 1);
    }
    string.into_iter().collect()
}

fn factorial(x: usize) -> usize {
    if x == 1 {
        x
    } else {
        x * factorial(x - 1)
    }
}

struct KnownLetter {
    l: char,
    pos: usize,
}

fn remove_known(s: &mut String, l: char, num: usize) -> Option<KnownLetter> {
    s.remove(s.find(l)?);

    Some(KnownLetter { l, pos: num })
}

// Modifies in place
fn restore_known(s: &mut String, k: &KnownLetter) {
    s.insert(k.pos, k.l);
}

fn benchmarks(base_height: usize) {
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
                THREADS);

        for t in 0..THREADS {
            let tmp_key_b = tmp_key.clone();
            
            let block = &max / THREADS;
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
}


fn smoke_pct() {
    let max_permutations = factorial(PZL_KEY.len());
    println!("[ pct{} :: {} ]", MAN_FILE, PZL_KEY);
    println!("base: {}", PZL_KEY.chars().count());
    println!("max: {} \n", max_permutations.to_formatted_string(&Locale::en));
    let start = Instant::now();
    let mut threads = vec![];


    for t in 0..THREADS {
        let sums = dump_manifest();
        let mut tmp_key = String::from(PZL_KEY);
        let b2b = Blake2bSum::new(64);
        let known0 = remove_known(&mut tmp_key, 'R', 0).unwrap();
        let known1 = remove_known(&mut tmp_key, 'Y', 2).unwrap();
        let known2 = remove_known(&mut tmp_key, 'U', 13).unwrap();
        let new_max = factorial(tmp_key.len());
        
        let block = &new_max / THREADS;
        let max = block + (block * t);
        let min = max - block;
        
        println!("thread: ({}) {} as base {}: {}", 
                t, PZL_KEY, tmp_key.chars().count(), tmp_key);
        println!("\t[min: {:>16}\tmax: {:>16}]", 
                min.to_formatted_string(&Locale::en), 
                max.to_formatted_string(&Locale::en));

        let thread_block = thread::spawn(move || { 
            for k in min..max {
                let mut x = permute(k, tmp_key.chars().collect());
                restore_known(&mut x, &known0);
                restore_known(&mut x, &known1);
                restore_known(&mut x, &known2);
                // println!("x: {}", x);
                // println!(" {:16}/{}: {}:{} ", k, max, x, start.elapsed().as_secs_f32());

                let check = b2b.read_str(x.clone() + "\n");

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
            "logs/timers_".to_owned() + MAN_FILE + ".log").expect("My dumbass error");

    log_file.write( format!("{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", 
            "Key:", PZL_KEY, 
            "Threads:", THREADS, 
            "Base:", PZL_KEY.chars().count(), 
            "Time:", start.elapsed().as_millis()).as_bytes() ).unwrap();

}


fn main() {
    smoke_pct();
    benchmarks(13);

}
