//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//! ## [Chapter 1: Basics of Rust Concurrency](https://marabos.nl/atomics/basics.html#interior-mutability)
//!
//! - Classic Cells [cages, containers of types]
//!   - `Cell`
//!     - inner can't be shared (memswaps and re-writes only)
//!   - `RefCell`
//!     - inner readshare/writeexclusive checked at runtime
//!     - panics on violation
//!   - `OnceCell`
//!     - `LazyCell` is common-case specialized variant where there is a global init pattern
//!     - readshare-only once init'd; can only be init'd once
//!     - use cases are different than classic inner-mute
//!   - `UnsafeCell`
//!     - unsafe
//! - Concurrent Cells
//!   - `Mutex`
//!     - inner can't be shared
//!   - `RwLock`
//!     - inner readshare/writeexclusive checked at runtime
//!     - blocks & waits to prevent violations
//!     - (usually blocks new readers if queued writer request)
//!   - `OnceLock`
//!     - `LazyLock` is common-case specialized variant where there is a global init pattern
//!     - readshare-only once init'd; can only be init'd once
//!     - use cases are different than classic inner-mute
//!   - `Atomics`
//!     - platform-specific primitive atomic operations
//!     - all ops require an `Ordering` value to be passed
//!     - all shares are of references
//!

use std::{thread, time::Duration};

use owo_colors::OwoColorize as _;

fn main() {
        // Cell
        {
                use std::cell::Cell;

                let cell = Cell::new(0);
                println!("cell.get() = {}", cell.get());
                cell.set(1);
                println!("- cell.set (1) ->");
                println!("cell.get() = {}", cell.get());
                let mut holder = 17;
                println!("holder = {}", holder);
                holder = cell.replace(holder);
                println!("- holder = cell.replace(holder) ->");
                println!("cell.get() = {}", cell.get());
                println!("holder = {}", holder);
        }
        println!("------");

        // RefCell
        {
                use std::cell::RefCell;

                let refcell = RefCell::new(0);
                println!("refcell.borrow() = {}", refcell.borrow());
                *refcell.borrow_mut() = 1;
                println!("- refcell.borrow_mut() = 1 ->");
                println!("refcell.borrow() = {}", refcell.borrow());
        }
        println!("------");

        // UnsafeCell
        {
                use std::cell::UnsafeCell;

                let unsafe_cell = UnsafeCell::new(0);
                // SAFETY: writing a direct value through an `UnsafeCell`
                // here is safe because we have exclusive access and know
                // there are no other references
                unsafe {
                        *unsafe_cell.get() = 1;
                }
                println!("- unsafe_cell.get() = 1 ->");
                // `.get()` yields a dereferenced mut value; using `into_inner()` for simplicity here
                println!("unsafe_cell.into_inner() = {}", unsafe_cell.into_inner());
        }
        println!("------");

        // OnceCell & LazyCell
        {
                use std::cell::{LazyCell, OnceCell};

                let celluno = OnceCell::new();
                println!("celluno.get() = {:?}", celluno.get());
                println!("celluno.get_or_init(|| 1) = {}", celluno.get_or_init(|| 1));
                println!("celluno.get_or_init(|| 2) = {}", celluno.get_or_init(|| 2));
                println!("celluno.get_or_init(|| 3) = {}", celluno.get_or_init(|| 3));

                let lazcelluno = LazyCell::new(|| 4);
                println!("lazcelluno = {:?}", lazcelluno);
                println!("{}lazcelluno = {:?}", "*".green(), *lazcelluno);
                println!("{}lazcelluno = {:?}", "*".green(), *lazcelluno);
                println!("{}lazcelluno = {:?}", "*".green(), *lazcelluno);
        }
        println!("------");
        println!("--concurrent options--");
        println!("------");

        // Mutex
        {
                use std::{sync::Mutex, thread, time};

                let time = time::Instant::now();
                let n = Mutex::new(0);
                println!("n.lock().unwrap() = {:?}", n.lock().unwrap());
                thread::scope(|s| {
                        for _ in 0..10 {
                                s.spawn(|| {
                                        let mut guard = n.lock().unwrap();
                                        print!("  {:<4}", guard);
                                        for _ in 0..100 {
                                                print!("+1");
                                                *guard += 1;
                                        }
                                        println!();
                                        drop(guard); // **NOTE**: without this `drop` we will wait on the sleep of all the threads
                                        thread::sleep(time::Duration::from_millis(500));
                                });
                        }
                        // **NOTE**: the scope block the main thread until it finishes, meaning the main thread will wait the duration of at least a single `sleep` call
                });
                println!("n.lock().unwrap() = {:?}", n.lock().unwrap());
                println!("time.elapsed() = {:?}", time.elapsed());
        }
        println!("------");

        // RwLock
        {
                use std::sync::RwLock;

                let rwl = RwLock::new(0);
                let rwl_ref = &rwl;
                thread::scope(|s| {
                        for i in 0..30 {
                                if i % 7 == 0 {
                                        s.spawn(move || {
                                                let mut guard = rwl_ref.write().unwrap();
                                                *guard += 1;
                                                println!("\ni ({}) div by 7 so rwl +1", i);
                                        });
                                } else {
                                        s.spawn(move || {
                                                let guard = rwl_ref.read();
                                                print!("  i: {} irwl.read() = {:?}  ", i, guard);
                                        });
                                }
                        }
                });
                println!();
        }
        println!("------");
        // Atomics
        {
                use std::sync::atomic::{AtomicUsize, Ordering};

                let atomic = AtomicUsize::new(0);
                println!("atomic.load() = {:?}", atomic.load(Ordering::SeqCst));
                thread::scope(|s| {
                        s.spawn(|| {
                                for i in 0..100 {
                                        if i % 17 == 0 {
                                                atomic.fetch_add(i, Ordering::SeqCst);
                                                println!(
                                                        "    from thread: {:?} + {}... atomic.load() = {:?}",
                                                        thread::current().id().green(),
                                                        i,
                                                        atomic.load(Ordering::SeqCst)
                                                );
                                        }
                                }
                        });
                        s.spawn(|| {
                                for i in 0..100 {
                                        if i % 7 == 0 {
                                                atomic.fetch_add(i, Ordering::SeqCst);
                                                println!(
                                                        "    from thread: {:?} + {}... atomic.load() = {:?}",
                                                        thread::current().id().blue(),
                                                        i,
                                                        atomic.load(Ordering::SeqCst)
                                                );
                                        }
                                }
                        });
                });
                println!("atomic.load() = {:?}", atomic.load(Ordering::SeqCst));
        }
        // OnceLock & LazyLock
        {
                use std::sync::{LazyLock, OnceLock};

                for _ in 0..3 {
                        let once_lockos: OnceLock<i32> = OnceLock::new();
                        println!("OnceLock not initialized: {:?}", once_lockos.get());
                        thread::scope(|s| {
                                for i in 0..12 {
                                        let once_lockos_ref = &once_lockos;
                                        s.spawn(move || {
                                                thread::sleep(Duration::from_millis(100));
                                                let val = once_lockos_ref.get_or_init(|| {
                                                        let init_with = i;
                                                        println!("    Initializing with value {}", init_with);
                                                        init_with
                                                });
                                                print!("    OnceLock value: {:?}", val);
                                        });
                                }
                        });
                        println!("OnceLock value: {:?}", once_lockos.get());
                }

                println!("\n");
                let lazlocker: LazyLock<i32> = LazyLock::new(|| 1121);
                println!("LazyLock value: {:?}", lazlocker.magenta());
                thread::scope(|s| {
                        for _ in 0..3 {
                                s.spawn(|| {
                                        thread::sleep(Duration::from_millis(100));
                                        println!("    lazyLock value: {:?}", (*lazlocker).blue());
                                });
                        }
                });
                println!("OnceLock value: {:?}", lazlocker.magenta());
        }
        println!("------");
}
