use std::{fs::File, io::{Read, Write}, collections::HashSet, thread, time::{Instant}};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};

// Initial Data
const THREADS: usize = 16;
const PZL_KEY: &str = "EGHMNPRTUWX";
const MAN_FILE: &str = "8";

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

// HNTUXERPGMW
fn main() {
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
        let known = remove_known(&mut tmp_key, 'H', 0).unwrap();
        let known1 = remove_known(&mut tmp_key, 'U', 3).unwrap();
        let known2 = remove_known(&mut tmp_key, 'N', 1).unwrap();
        println!("thread: ({}) for {}", t, tmp_key);

        let block = &max_permutations / THREADS;
        let max = block + (block * t);
        let min = max - block;

        println!("\tmin: {}\tmax: {}", min, max);
        let thread_block = thread::spawn(move || { 
            for k in min..max {
                let mut x = permute(k, tmp_key.chars().collect());
                restore_known(&mut x, &known);
                restore_known(&mut x, &known1);
                restore_known(&mut x, &known2);
                //println!("x: {}", x);
                // println!(" {:16}/{}: {}:{} ", k, max, x, start.elapsed().as_secs_f32());

                let check = b2b.read_str(x.clone() + "\n");

                if sums.contains(&check) {
                    println!("Found solution: {} [took {}ms]", x, start.elapsed().as_millis());
                    break;
                }
                // thread::sleep(Duration::from_millis(50));
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
    println!("  . iters: {}", max_permutations);
    println!("  . threads: {}", THREADS);
    println!("  . time: {}ms", start.elapsed().as_millis());

    let mut log_file = File::options().append(true).create(true).open("logs/timers").expect("My dumbass error");

    log_file.write( format!("{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", "Key:", PZL_KEY, "Threads:", THREADS, "Base:", PZL_KEY.chars().count(), "Time:", start.elapsed().as_millis()).as_bytes() ).unwrap();


}
