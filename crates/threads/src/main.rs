//! Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!

mod error;
use std::thread;

use crate::error::ErrWrapper;
pub type Result<T> = std::result::Result<T, ErrWrapper>;
use owo_colors::OwoColorize;

fn main() {
        for _ in 0..5 {
                thread::spawn(f);
        }
        println!("{} from the {} thread.", "Hello".cyan(), "main".blue());
}

fn f() {
        println!("{} from {} thread!", "Hello".cyan(), "another".green());
        let id = thread::current().id();
        println!("This is my thread id: {:?}", id.purple());
}
