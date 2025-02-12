//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! ## Threads spawned with and without await.
//! ## **NOTE**
//! use of `std::io::Stdout::lock()` is used in `println!()`, resulting in atomic-like writing.
//! Notably: thread id is not always written.
//! I've not seen any other withihn `f()` joints, but that may just be a statistics issue.
//! *Likely* the lock just prevents interleaving, but some other dynamics relating to writing to stdout define what sorts of behavior can occur at thread close
//! boundaries.  (Q: what chars can be produced? Is stdout doing any sanitation on binary data written to it?)

use std::thread;

use clap::Parser;
use owo_colors::OwoColorize;

/// interface for scratch code for use with [Rust Atomics and Locks](https://marabos.nl/atomics/)
#[derive(Parser, Debug)]
#[command(version, about, long_about, disable_help_subcommand = true, subcommand_help_heading = "input source")]
struct Args {
        /// number of threads to spawn
        #[arg(default_value = "3")]
        threads: usize,
        /// wait on threads at end of main
        #[arg(short, long)]
        wait_on: bool,
        /// number of times to repeat main{}
        #[arg(short, long, default_value = "0")]
        repeats: usize,
}
fn main() {
        let args = Args::parse();
        dbg!(&args);
        for _ in 0..1 + args.repeats {
                println!("--------------------------");
                let mut handles = vec![];
                for _ in 0..args.threads {
                        let h = thread::spawn(f);
                        handles.push(h);
                }
                println!("{} from the {} thread.", "Hello".cyan(), "main".blue());
                if args.wait_on {
                        for h in handles {
                                h.join().unwrap();
                        }
                }
        }
}

fn f() {
        println!("{} from {} thread!", "Hello".cyan(), "another".green());
        let id = thread::current().id();
        println!("This is my thread id: {:?}", id.purple());
}
