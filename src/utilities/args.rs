use std::{env::Args, process::exit};

use super::knowns::KnownLetter;

#[derive(Default)]
pub struct Opts {
    pub thread_count: usize,
    pub letters: String,
    pub pct_x: String,
    pub known_letters: Option<Vec<KnownLetter>>,
    pub verbosity: usize,
}

fn get_arg(args: &Vec<String>, i: usize, c: char) -> String {
    if i >= args.len() {
        panic!("Missing argument for option {}", c)
    }
    args[i].clone()
}

fn get_long_opt(_opts: &mut Opts, arg: &String) {
    match arg.as_str() {
        "--help" => help_me(),
        x => panic!("Invalid long option: {}", x)
    }
}
fn get_short_opt(opts: &mut Opts, args: &Vec<String>, i: usize) {
    let mut offset = 0;
    for c in args[i].chars().skip(1) {
        match c {
            '-' => {
                get_long_opt(opts, &args[i]);
                return
            },
            'h' => help_me(),
            't' => {
                offset += 1;
                let arg = get_arg(&args, i + offset, c);
                opts.thread_count = usize::from_str_radix(&arg, 10).unwrap()
            },
            'l' => {
                offset += 1;
                opts.letters = get_arg(&args, i + offset, c);
            },
            'x' => {
                offset += 1;
                opts.pct_x = get_arg(&args, i + offset, c);
            },
            x => panic!("Invalid option: {}", x)
        }
    }
}

fn check_opts(opt: &mut Opts) {
    let mut message = String::new();
    let mut is_error = false;
    if opt.thread_count < 1 {
        is_error = true;
        message.push_str("Invalid number of threads. ");
    }
    if opt.letters.is_empty() {
        is_error = true;
        message.push_str("Must provide puzzle letters. ");
    }
    if opt.pct_x.is_empty() {
        is_error = true;
        message.push_str("Must provide manifest path. ");
    }
    if is_error {
        println!("Error: {message}");
        useage();
        exit(0);
    }
}

// Eats 'args'
pub fn get_options(args: Args) -> Opts {
    let mut opts = Opts::default();
    let args: Vec<String> = args.collect();
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with('-') {
            get_short_opt(&mut opts, &args, i);
        }
    }

    // Check opts
    check_opts(&mut opts);

    opts
}

// Remember to explicitly call exit after if you so desire!
fn useage() {
    let message = 
    "Useage: smokepct [-t : thread count] [-l : puzzle letters] [-x : manifest path]";
    println!("{message}");
}

fn help_me() {
    let message = 
    "smokepct - by Dan Muck and Patrick LeBlanc

    Options:
    -h/--help: Display this message
    -t:        Thread count
    -l:        The original key to iterate. Found in a pct's \"letter: \" section
    -x:        pctX puzzle number (0, 1, ..., A, B). The program will look for a manifest
               file under MANIFEST/X, where X is the puzzle number (MANIFEST/1, MANIFEST/A)
               The manifest file for any given pct is found at:
               /var/public/SEMESTER/CLASS/pct#/MANIFEST";

    println!("{message}");
    useage();
    exit(0);
}