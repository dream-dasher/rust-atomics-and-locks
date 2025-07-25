//! # Scratch code for [Rust Atomics and Locks](https://marabos.nl/atomics/)
//!
//! ## [Chapter 1: Basics of Rust Concurrency](https://marabos.nl/atomics/basics.html#waiting)
//!
//! - Parking
//! - Condition Variables
//!   - take a mutex
//!   - notify_all vs notify_one

use std::{collections::VecDeque, sync::Mutex, thread, time::Duration};

use owo_colors::OwoColorize;
fn main() {
       {
              println!("\n-----{}-----", "Thread Parking".bold().purple());
              const END_VALUE: usize = 12;

              let queue = Mutex::new(VecDeque::new());
              thread::scope(|s| {
                     // consuming thread
                     let consumer = s.spawn(|| {
                            loop {
                                   let item = queue.lock().unwrap().pop_front();
                                   if let Some(item) = item {
                                          dbg!(&item);
                                          if item == END_VALUE {
                                                 break;
                                          }
                                   } else {
                                          thread::park();
                                   }
                            }
                     });

                     // producer (in main thread)
                     for i in 0..=END_VALUE {
                            queue.lock().unwrap().push_back(i);
                            consumer.thread().unpark();
                            thread::sleep(Duration::from_millis(70));
                     }
                     consumer.join().unwrap();
              });
       }
       {
              use std::sync::Condvar;
              println!("\n-----{}-----", "Condition Variables".bold().purple());
              const END_VALUE: usize = 12;

              let queue = Mutex::new(VecDeque::new());
              let not_empty_condvar = Condvar::new();

              thread::scope(|s| {
                     s.spawn(|| {
                            loop {
                                   let mut q = queue.lock().unwrap();
                                   let item = loop {
                                          if let Some(item) = q.pop_front() {
                                                 break item;
                                          } else {
                                                 q = not_empty_condvar.wait(q).unwrap();
                                          }
                                   };
                                   drop(q);
                                   dbg!(&item);
                                   if item == END_VALUE {
                                          break;
                                   }
                            }
                     });

                     for i in 0..=END_VALUE {
                            queue.lock().unwrap().push_back(i);
                            not_empty_condvar.notify_one();
                            thread::sleep(Duration::from_millis(70));
                     }
              })
       }
}
