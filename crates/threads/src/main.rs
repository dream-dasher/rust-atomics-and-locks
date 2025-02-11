//! Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!

mod error;
use crate::error::ErrWrapper;
pub type Result<T> = std::result::Result<T, ErrWrapper>;

use tracing as tea;
use utilities::activate_global_default_tracing_subscriber;

fn main() -> Result<()> {
        let _writer_guard = activate_global_default_tracing_subscriber()
                .maybe_env_default_level(None)
                .maybe_trace_error_level(None)
                .call()?;
        let start_time = std::time::Instant::now();
        tea::info!("hi there");

        let total_time_elapsed = start_time.elapsed();
        tea::info!(?total_time_elapsed);

        Ok(())
}
