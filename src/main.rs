//! Implementation of the Range Minimum Query algorithms presented in [1].
//!
//! [1] M.A. Bender, M. Farach-Colton:
//!     The LCA Problem Revisited.
//!     LATIN 2000, LNCS 1776, 88-94, 2000.

use std::fmt::Write;

mod log;

/// Converts the given time (in milliseconds) into the a string using appropriate unit.
fn print_time(time: u64) -> String {
    const MIL_SEC: u64 = 1;
    const DEC_SEC: u64 = 100 * MIL_SEC;
    const SECOND: u64 = 10 * DEC_SEC;
    const MINUTE: u64 = 60 * SECOND;
    const HOUR: u64 = 60 * MINUTE;
    const DAY: u64 = 24 * HOUR;

    let mut time_str = String::new();

    if time < SECOND {
        write!(time_str, "{time: >3} ms").unwrap();
    } else if time <= MINUTE {
        let mut d_secs = (time + DEC_SEC - 1) / DEC_SEC;
        let secs = d_secs / 10;
        d_secs = d_secs % 10;

        write!(time_str, "{secs: >2}.{d_secs} s").unwrap();
    } else if time <= HOUR {
        let mut secs = (time + SECOND - 1) / SECOND;
        let mins = secs / 60;
        secs = secs % 60;

        write!(time_str, "{mins: >2} min {secs: >2} s").unwrap();
    } else if time <= DAY {
        let mut mins = (time + MINUTE - 1) / MINUTE;
        let hours = mins / 60;
        mins = mins % 60;

        write!(time_str, "{hours: >2} h {mins: >2} min").unwrap();
    } else {
        let mut hours = (time + HOUR - 1) / HOUR;
        let days = hours / 24;
        hours = hours % 24;

        write!(time_str, "{days: >2} d {hours: >2} h").unwrap();
    }

    time_str
}

fn main() {
    const DATA_SIZE: usize = 20000;
    const QUERIES: usize = 1000000;
    const SEED: u64 = 19082017;

    println!("   Size: {}", DATA_SIZE);
    println!("Queries: {}", QUERIES);
    println!();

    let ref_time;

    println!("*** Reference ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0));
        println!("Q: {}", print_time(time_pair.1));
        println!();

        ref_time = time_pair
    }

    println!("*** No Pre-Processing ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));
        println!();
    }

    println!("*** Naive ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!("*** Segment Tree ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!("*** Segment Tree Cache ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!("*** Sparse Table ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!("*** Plus Minus 1 ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!("*** RMQ via +-1 LCA ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));

        // Verify correctness.
        // ToDo: Implement and run algorithm.
        let correct = true;

        println!("C: {}", if correct { "Yes" } else { "No" });
        println!();
    }

    println!();
    println!(" --- --- Testing LCA Algorithms. --- ---");

    let ref_time;

    println!("*** Reference ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0));
        println!("Q: {}", print_time(time_pair.1));
        println!();
    
        ref_time = time_pair;
    }

    println!("*** Segment Tree Cache ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));
        println!();
    }

    println!("*** Sparse Table ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));
        println!();
    }

    println!("*** Plus Minus 1 ***");
    {
        // ToDo: Implement and run algorithm.
        let time_pair = (0, 0);

        println!("P: {}", print_time(time_pair.0 - ref_time.0));
        println!("Q: {}", print_time(time_pair.1 - ref_time.1));
        println!();
    }
}
