//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//! ## [Chapter 2: Atomics](https://marabos.nl/atomics/atomics.html#example-stop-flag)
//!
//! - Load, Store
//! - Fetch_&_Modify
//! - Compare_&_Exchange

use std::{sync::atomic::{AtomicBool, AtomicUsize, Ordering::Relaxed},
          thread};

use owo_colors::{Color, OwoColorize as _, XtermColors};

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
                println!("\n-----{}-----", "Fetch_&_Modify: Synchronization".bold().purple());
                const NUM_THREADS: usize = 50;
                const ADDS_PER_THREAD: usize = 100;
                let atomic_num_done = &AtomicUsize::new(0);
                let main_thread_handle = &thread::current(); // for unparking
                thread::scope(|s| {
                        // 'background thread' processing 100 items
                        for t in 0..NUM_THREADS {
                                s.spawn(move || {
                                        let thread_color = XtermColors::from(t as u8);
                                        for _ in t..(t + ADDS_PER_THREAD) {
                                                thread::sleep(std::time::Duration::from_millis(2)); // fake processing
                                                atomic_num_done.fetch_add(1, Relaxed);
                                                main_thread_handle.unpark(); // wake main up
                                                print!(
                                                        "+{}",
                                                        "1".color(thread_color), // auto-assign colors j
                                                );
                                        }
                                });
                        }
                        loop {
                                let current_done = atomic_num_done.load(Relaxed);
                                println!(
                                        "\nProcessed {}/{} items",
                                        current_done.to_string().blue(),
                                        NUM_THREADS * ADDS_PER_THREAD
                                );
                                if current_done >= NUM_THREADS * ADDS_PER_THREAD {
                                        println!("{}", "All items processed".green());
                                        break;
                                } else {
                                        thread::park(); // for efficiency
                                }
                        }
                });
        }
}
