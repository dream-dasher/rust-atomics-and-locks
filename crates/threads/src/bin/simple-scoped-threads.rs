//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! ## [Chapter 1: Basics of Rust Concurrency](https://marabos.nl/atomics/basics.html#scoped-threads)
//!
//! `scope(f)` takes a function with a `Scope` object as its argument.   This allows the compiler to infer the type of of `s` as `Scope`.
//! Additionally `scope(f)` *generates* a scope object which ... being the only one I suppose the compiler also assumes is what's captured.
//!
//! ## NOTE
//! Semi-recent edition (`thread::scope(||..)`)
//! It is **not** guaranteed that objects will be dropepd at the end of their lifetime.
//! This invalidated a previous "guard" based implementation, as lifetimes ordering couldn't be guaranteed in the case of a leak.
//! (re: "The Leakpocalypse")

use std::thread;

use owo_colors::OwoColorize;

fn main() {
       println!("\n-----{}-----", "Scoped Threads".bold().purple());
       let numbers = [0, 1, 2, 3, 4];
       // `scope(f)` takes a function with a `Scope` object as its argument.   This allows the compiler to infer the type of of `s` as `Scope`.
       // Additionally `scope(f)` *generates* a scope object which ... being the only one I suppose the compiler also assumes is what's captured.
       thread::scope(|s| {
              s.spawn(|| {
                     println!("length: {}", numbers.len().cyan());
              });
              s.spawn(|| {
                     println!("sum: {}", numbers.iter().sum::<i32>().magenta());
              });
              s.spawn(|| {
                     println!("product (exclude `0`): {}", numbers.iter().filter(|&&n| n > 0).product::<i32>().yellow());
              });
              s.spawn(|| {
                     println!("The numbers: {:?}", numbers.on_black());
              });
       });
}
