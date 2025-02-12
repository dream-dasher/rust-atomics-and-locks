//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! ## Threads taking closures.
//! ## **NOTE**
//!  `move` required even with `.join()`
//! Scoped closures can get around this, but I'm not yet sure why the `.join() is insufficient - though the return of a `Result<>` is likely a clue.

use std::thread;

use owo_colors::OwoColorize;

fn main() {
        let to_sum = Vec::from_iter(0..=1000);
        let t = thread::spawn(move || {
                let sum: isize = to_sum.iter().sum();
                sum
        });
        let sum = t.join().unwrap();
        println!("The sum of 0 to 1000 is: {}", sum.green());

        let numbers_1 = vec![0, 1, 2, 3, 4];
        thread::spawn(move || {
                for n in &numbers_1 {
                        println!("number: {}", n.green());
                }
        });

        let numbers_2: Vec<i32> = (20..29).collect();
        thread::spawn(move || {
                for n in &numbers_2 {
                        println!("number: {}", n.blue());
                }
        });

        let numbers_3: Vec<i32> = (30..39).collect();
        thread::spawn(move || {
                for n in &numbers_3 {
                        println!("number: {}", n.yellow());
                }
        });
        // std::thread::sleep(std::time::Duration::from_millis(1));
        println!("{} from {}", "hi there".purple(), "main".blue());
}
