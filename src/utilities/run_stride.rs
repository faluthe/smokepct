use std::{fs::File, io::Write, thread, time::Instant};
use b2sum_rust::Blake2bSum;
use num_format::{ToFormattedString, Locale};
use ansi_term::Colour::{Yellow, Blue, Purple, Cyan, Green};

use crate::utilities::{permute, factorial, dump_manifest};
use crate::utilities::knowns::{remove_knowns, restore_knowns};

use crate::utilities::args::{Opts};
use crate::{DEBUG, LOGS};

use super::knowns::populate_knowns;

pub fn run_stride(arguments: &Opts) {
    let thread_count = arguments.thread_count;
    let letters = arguments.letters.clone();
    let pct_x = arguments.pct_x.clone();
    let max_permutations = factorial(letters.len());

    let start = Instant::now();
    let mut threads = vec![];

    let mut as_base: usize = 0;
    let mut as_max: usize = 0;
    if DEBUG > 0 {
        // Console Output
        println!("\n{} {}{} :: {} {}",
            Yellow.bold().paint("["),
            Purple.bold().paint("pct"),
            Purple.bold().paint(&pct_x), 
            Purple.bold().paint(&letters),
            Yellow.bold().paint("]")
        );
        println!(" . {} {}",
            Yellow.paint("base:"),
            Purple.bold().paint(letters.chars().count().to_string())
        );
        println!(" . {} {} \n",
            Yellow.paint("max:"),
            Purple.bold().paint(max_permutations.to_formatted_string(&Locale::en))
        );
    }

    let mut i = 0;
    let strides_vec: Vec<String> = arguments.stride.to_owned();
    for stride_str in strides_vec {
        i = i + 1;
        let sums = dump_manifest(String::from("MANIFEST/".to_owned() + &pct_x));
        let b2b = Blake2bSum::new(64);

        let mut tmp_key = letters.clone();
        
        remove_knowns(&mut tmp_key, &populate_knowns(&stride_str));
        let new_max = factorial(tmp_key.len());
        
        let smoked_as_base = tmp_key.chars().count();
        if DEBUG > 0 {
            println!("{} {:<3} {} {}{}{} {} as base {}: {}",
            Blue.dimmed().paint(format!("{:<3}","thread:")),
            Blue.dimmed().paint(i.to_string()),
            Blue.dimmed().paint("::"),
            Blue.dimmed().paint("("),
            Blue.paint(&stride_str.to_string()),
            Blue.dimmed().paint(")"),
            Green.bold().paint(&letters), 
            Purple.bold().paint(tmp_key.chars().count().to_string()), 
            Purple.paint(&tmp_key)
        );
        
    }
    
    let thread_block = thread::spawn(move || { 
        for k in 0..new_max {
            let mut x = permute(k, tmp_key.chars().collect());
            restore_knowns(&mut x, &populate_knowns(&stride_str));
            let check = b2b.read_str(x.clone() + "\n");

                if DEBUG > 4 {
                    println!("{}", Green.paint("@single_permute-------->"));
                    println!("x: {}", x);
                    println!("{}", Green.paint("<------------------------\n"));

                }

                if sums.contains(&check) {
                    if DEBUG > 0 {
                        println!("{} {} [took {}ms]",
                            Cyan.bold().paint("Found solution:"), 
                            Cyan.bold().paint(&x),
                            start.elapsed().as_millis());
                    }
                    break;
                }
            }
        });
        threads.push(thread_block);
        as_base = smoked_as_base;
        as_max = new_max;
    }
    for thread in threads {
        let _ = thread.join().unwrap();
    }

    if DEBUG > 0 {
        println!("\n . . \n");
        println!(" [ pct{} :: {} ]", pct_x, letters);
        println!("  . puzzle_base: {}", letters.chars().count());
        println!("  . base_permutes: {}", max_permutations.to_formatted_string(&Locale::en));
        println!("  . smoked_as_base: {}", as_base);
        println!("  . actual_permutes: {}", as_max.to_formatted_string(&Locale::en));
        println!("  . threads: {}", thread_count);
        println!("  . time: {}ms", start.elapsed().as_millis());
    }

    if LOGS == true {
        crate::utilities::new_dir("logs").unwrap();
        let mut log_file = File::options().append(true).create(true).open(
                "logs/timers_".to_owned() + &pct_x + ".stride.log")
                .expect("Error creating log_file");
    
        log_file.write(format!(
                "{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}\n{:9} {}ms\n\n", 
                "Smoked as Base:", as_base,
                "Key:", letters, 
                "Base:", letters.chars().count(),
                "Threads:", thread_count, 
                "Time:", start.elapsed().as_millis() 
            ).as_bytes()).unwrap();
    }
}
