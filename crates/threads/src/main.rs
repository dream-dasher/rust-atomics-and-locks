//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)

mod error;

use crate::error::ErrWrapper;
pub type Result<T> = std::result::Result<T, ErrWrapper>;

use std::thread;

use owo_colors::OwoColorize;

fn main() -> Result<()> {
       thread::Builder::new()
              .name("First non-main".into())
              .stack_size(1024)
              // .no_hooks()
              .spawn(f)?; // Note: this spawn allows error handling unlike default thread::spawn
       std::thread::sleep(std::time::Duration::from_secs(1));
       println!("{} from the {} thread.", "Hello".cyan(), "main".blue());

       Ok(())
}

fn f() {
       println!("{} from {} thread!", "Hello".cyan(), "another".green());
       let id = thread::current().id();
       println!("This is my thread id: {:?}", id.purple());
       let name = thread::current().name().unwrap().to_string();
       println!("This is my thread name: {:?}", name.purple());
}
