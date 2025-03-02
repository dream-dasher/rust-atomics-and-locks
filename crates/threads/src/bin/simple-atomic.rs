//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//! ## [Chapter 2: Atomics](https://marabos.nl/atomics/atomics.html#example-stop-flag)
//!
//! - Load, Store
//! - Fetch_&_Modify
//! - Compare_&_Exchange

use std::{sync::atomic::{AtomicBool, AtomicI64, AtomicIsize, AtomicUsize, Ordering::Relaxed},
          thread};

use owo_colors::{OwoColorize as _, XtermColors};

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
                let atomic_max_diff = &AtomicUsize::new(0);
                let main_thread_handle = &thread::current(); // for unparking
                thread::scope(|s| {
                        // 'background thread' processing 100 items
                        for t in 0..NUM_THREADS {
                                s.spawn(move || {
                                        let thread_color = XtermColors::from(t as u8);
                                        let mut max_diff: usize = 0;
                                        let mut last_counter_value = 0;

                                        for _ in t..(t + ADDS_PER_THREAD) {
                                                thread::sleep(std::time::Duration::from_millis(2)); // fake processing
                                                // fetch_add & get current value of counter
                                                let incoming_counter_value = atomic_num_done.fetch_add(1, Relaxed);

                                                // calculate max diff observed between `num_done` counter observations
                                                let curr_diff = incoming_counter_value
                                                        .checked_sub(last_counter_value)
                                                        .expect("values should be monotonic increasing");
                                                if curr_diff > max_diff {
                                                        max_diff = max_diff.max(curr_diff);
                                                        atomic_max_diff.fetch_max(curr_diff, Relaxed);
                                                }
                                                last_counter_value = incoming_counter_value;

                                                // let wake main thread (not really needed given the rapid timing of this example (I assume ..(?)))
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
                                        "\nProcessed {}/{} items -- Max diff: {}",
                                        current_done.to_string().blue(),
                                        NUM_THREADS * ADDS_PER_THREAD,
                                        atomic_max_diff.load(Relaxed).green()
                                );
                                if current_done >= NUM_THREADS * ADDS_PER_THREAD {
                                        println!("{}", "All items processed".green());
                                        println!("Max diff: {}", atomic_max_diff.load(Relaxed).green().bold());
                                        break;
                                } else {
                                        thread::park(); // for efficiency
                                        // thread::park_timeout(Duration::from_millis(1000)); // were updates much slower
                                }
                        }
                });
        }
}
