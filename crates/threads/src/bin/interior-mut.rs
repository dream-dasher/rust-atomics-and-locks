//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! - Cells [cages, containers of types]
//!   - `Cell`
//!     - inner can't be shared
//!   - `RefCell`
//!     - inner sharing checked at runtime
//!   - `UnsafeCell`
//!     - unsafe
//!   - `OnceCell`
//!     - share-borrow only once init'd; can only be init'd once
//!     - only *sorta* 'inner mut' -- *usually* it's way to immutably share, but perform an init if needed (which is often acontextual in content, but not in principle).
//! - Concurrent Cells
//!   - `Mutex`
//!   - `RwLock`
//!

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

        // OnceCell
        {
                use std::cell::OnceCell;
                let celluno = OnceCell::new();
                println!("celluno.get() = {:?}", celluno.get());
                println!("celluno.get_or_init(|| 1) = {}", celluno.get_or_init(|| 1));
                println!("celluno.get_or_init(|| 2) = {}", celluno.get_or_init(|| 2));
                println!("celluno.get_or_init(|| 3) = {}", celluno.get_or_init(|| 3));
        }
        println!("------");

        // Mutex
        {}
        println!("------");

        // RwLock
        {}
        println!("------");
}
