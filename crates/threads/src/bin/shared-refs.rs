//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! ## [Chapter 1: Basics of Rust Concurrency](https://marabos.nl/atomics/basics.html#shared-ownership-and-reference-counting)
//!
//! - static variable initialized with const variables
//! - leak
//!   - **NOTE** 1: we need to explicitly note the type as `&'static` -- by default it is `&'static mut`, which can't be shared
//!   - **NOTE** 2: `mov(e)`ing a reference `cop(y)`ies it
//! - ref counting
//!   - **NOTE**: `Arc` variant is needed for our purposes (vs `Rc`)

use std::{sync::Arc, thread, time::Duration};

use owo_colors::OwoColorize;

fn main() {
        {
                println!("\n-----{}-----", "statics & constants for multithread use".magenta());
                // static variable initialized with const variables
                static STATOS_VAROS: [i32; 7] = [0, 1, 2, 3, 4, 5, 6];
                thread::spawn(|| println!("STATOS_VAROS: {:?}", STATOS_VAROS));
                thread::spawn(|| println!("STATOS_VAROS: {:?}", STATOS_VAROS));

                const CONSTOS_VAROS: [i32; 7] = [10, 11, 12, 13, 14, 15, 16];
                thread::spawn(|| println!("CONSTOS_VAROS: {:?}", CONSTOS_VAROS));
                thread::spawn(|| println!("CONSTOS_VAROS: {:?}", CONSTOS_VAROS));
        }
        thread::sleep(Duration::from_millis(100));
        {
                println!("\n-----{}-----", "leak for multithread use".magenta());
                // leak
                // **NOTE** 1: we need to explicitly note the type as `&'static` -- by default it is `&'static mut`, which can't be shared
                // **NOTE** 2: `mov(e)`ing a reference `cop(y)`ies it
                let vectoros = vec![20, 21, 22];
                let vectoros_leaked: &'static _ = Vec::leak(vectoros);
                thread::spawn(move || println!("vectoros: {:?}", vectoros_leaked));
                thread::spawn(move || println!("vectoros: {:?}", vectoros_leaked));

                let boxos = Box::new([30, 31, 32]);
                let boxos_leaked: &'static _ = Box::leak(boxos);
                thread::spawn(move || println!("boxos_leaked: {:?}", boxos_leaked));
                thread::spawn(move || println!("boxos_leaked: {:?}", boxos_leaked));
        }
        thread::sleep(Duration::from_millis(100));
        {
                println!("\n-----{}-----", "refcounting for multithread use".magenta());
                // ref counting
                // **NOTE**: `Arc` variant is needed for our purposes (vs `Rc`)
                let arc_count = Arc::new([50, 51]);
                thread::spawn({
                        let arc_count = arc_count.clone();
                        move || {
                                println!("arc-count: {:?}", arc_count);
                        }
                });
                thread::spawn({
                        let arc_count = arc_count.clone();
                        move || {
                                println!("arc-count: {:?}", arc_count);
                        }
                });
        }
        // sleep to let other threads run
        thread::sleep(Duration::from_millis(100));
        println!("----");
}
