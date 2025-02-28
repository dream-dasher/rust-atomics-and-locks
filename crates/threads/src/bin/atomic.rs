//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//! ## [Chapter 2: Atomics](https://marabos.nl/atomics/atomics.html#example-stop-flag)

use std::{sync::atomic::{AtomicBool, Ordering::Relaxed},
          thread};

fn main() {
        static STOP: AtomicBool = AtomicBool::new(false);

        // work 'till it sees atomic global is true
        let background_thread = thread::spawn(|| {
                while !STOP.load(Relaxed) {
                        thread::sleep(std::time::Duration::from_millis(100))
                }
                println!("`STOP==true` observed. Background thread stopping.");
        });

        // loop until break at which point cleanup
        for line in std::io::stdin().lines() {
                match line.unwrap().as_str() {
                        "help" => println!("Available commands: help, stop"),
                        "stop" => break,
                        cmd => println!("Unknown command: {:?}\ntry: \"help\"", cmd),
                }
        }
        STOP.store(true, Relaxed);
        background_thread.join().unwrap();
}
