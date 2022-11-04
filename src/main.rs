use std::{fs::File, io::Read, collections::HashSet, thread, time::{Instant, Duration}};

use b2sum_rust::Blake2bSum;

fn dump_manifest() -> HashSet<String> {
    let mut manifest = File::open("MANIFEST/1").expect("Manifest not found");
    let mut data = String::new();
    manifest.read_to_string(&mut data).unwrap();
    let mut sums: HashSet<String> = HashSet::new();
    
    for l in data.lines() {
        sums.insert(l.to_uppercase());
    }
    sums
}

fn permute(mut k: usize, mut s: Vec<char>) -> String {
    for i in 1..s.len() {
        s.swap(k % (i + 1), i);
        k = k / (i + 1);
    }
    s.into_iter().collect()
}

fn factorial(x: usize) -> usize {
    if x == 1 {
        x
    } else {
        x * factorial(x - 1)
    }
}

fn main() {
    let testkey = "HFXCWEAUTN";
    let max_permutations = factorial(testkey.len());
    println!("max: {}", max_permutations);
    let start = Instant::now();
    let mut threads = vec![];
    let thread_count = 8;

    for t in 0..thread_count {
        let sums = dump_manifest();
        let tmp_key = testkey;
        let b2b = Blake2bSum::new(64);
        println!("thread: ({}) for {}", t, tmp_key);

        let block = &max_permutations / thread_count;
        let max = block + (block * t);
        let min = max - block;

        println!("\tmin: {}\tmax: {}", min, max);
        let thread_block = thread::spawn(move || { 
            for k in min..max {
                let x = permute(k, testkey.chars().collect());
                // println!("{}", x);
                let check = b2b.read_str(x.clone() + "\n");

                if sums.contains(&check) {
                    println!("Found solution: {}", x);
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

    println!(" . . ");
    print!("time: {}", start.elapsed().as_millis());
}
