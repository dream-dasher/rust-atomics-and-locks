//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//! ## [Chapter 2: Atomics](https://marabos.nl/atomics/atomics.html#example-stop-flag)
//!
//! - Load, Store
//! - Fetch_&_Modify
//! - Compare_&_Exchange

use std::{sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
          thread};

use owo_colors::OwoColorize as _;

fn main() {
        static STOP: AtomicBool = AtomicBool::new(false);

        {
                println!("\n-----{}-----", "Load, Store: STOP signal.".bold().purple());
                // work 'till it sees atomic global is true
                let background_thread = thread::spawn(|| {
                        while !STOP.load(Relaxed) {
                                thread::sleep(std::time::Duration::from_millis(100))
                        }
                        println!("`{}=={}` observed. Background thread stopping.", "STOP".red(), "true".magenta());
                });

                println!("Type \"{}\" for a list of commands", "help".green());
                // loop until break at which point cleanup
                for line in std::io::stdin().lines() {
                        match line.unwrap().as_str() {
                                "help" => println!("Available commands: {}, {}", "help".green(), "stop".green()),
                                "stop" => break,
                                cmd => println!("Unknown command: {:?}\ntry: \"{}\"", cmd.blue(), "help".green()),
                        }
                }
                STOP.store(true, Relaxed);
                background_thread.join().unwrap();
        }
        {
                println!("\n-----{}-----", "Single extraThread Update Sync".bold().purple());
                // this should really be a `fetch_add`
                let atomic_num_done = AtomicUsize::new(0);
                let main_thread_handle = thread::current(); // for unparking
                thread::scope(|s| {
                        // 'background thread' processing 100 items
                        s.spawn(|| {
                                for _ in 0..100 {
                                        thread::sleep(std::time::Duration::from_millis(10)); // fake processing
                                        let current_done = atomic_num_done.load(Relaxed);
                                        atomic_num_done.store(current_done + 1, Relaxed);
                                        main_thread_handle.unpark(); // wake main up
                                }
                        });
                        loop {
                                let current_done = atomic_num_done.load(Relaxed);
                                println!("Processed {}/100 items", current_done.blue());
                                if current_done >= 100 {
                                        println!("{}", "All items processed".green());
                                        break;
                                } else {
                                        thread::park(); // for efficiency
                                }
                        }
                });
        }
}
