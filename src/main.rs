use std::env;
// Local
mod envs;
mod utilities;
use utilities::smoke_pct::smoke_pct;
use utilities::run_stride::run_stride;
use utilities::{permute, factorial};
use utilities::unit_tests::dry_run;
use utilities::args::{self};

// Options
// . DEBUG = {0, 1, 2, 3, 4, 5} (level of verbosity)
const DEBUG: usize = 1;
const LOGS: bool = true;
const BENCH: bool = false;


fn main() {
    // Get arguments
    let arguments = args::get_options(env::args());
    
    if arguments.stride.capacity() > 0 {
        run_stride(&arguments);

    } else {
        smoke_pct(&arguments);

    }

    if BENCH == true {
        dry_run(arguments.letters.chars().count(), arguments.thread_count);
    }

}