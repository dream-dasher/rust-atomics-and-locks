//! Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!

mod error;
use std::thread;

use crate::error::ErrWrapper;
pub type Result<T> = std::result::Result<T, ErrWrapper>;
use owo_colors::OwoColorize;
use tracing as tea;

fn main() {
        // fn main() -> Result<()> {
        let start_time = std::time::Instant::now();
        // let _writer_guard = {
        //         utilities::activate_global_default_tracing_subscriber()
        //                 .maybe_env_default_level(None)
        //                 .maybe_trace_error_level(None)
        //                 .call()
        //                 .expect("Failed to set up tracing subscriber.")
        // };

        for _ in 0..500 {
                thread::spawn(f);
        }
        println!("{} from the {} thread.", "Hello".cyan(), "main".blue());

        let total_time_elapsed = start_time.elapsed();
        tea::info!(?total_time_elapsed);
        // Ok(())
}

fn f() {
        println!("{} from {} thread!", "Hello".cyan(), "another".green());
        let id = thread::current().id();
        println!("This is my thread id: {:?}", id.purple());
}
