# Scratch Repo to accompany [Rust Atomics and Locks](https://marabos.nl/atomics/basics.html)

## Threads  (to organize)
### Chapter 1: Basics of Rust Concurrency
- Threads
  - `simple-threads`
  - `simple-scoped-threads`
  - `thread-closure`
- Interior Mut & share structures
  - `interior-mut`
  - `shared-refs`
- Waiting
  - `park-and-convar`

### Chapter 2: Atomics
- Atomics 
  - Load, Store
  - Fetch-&-Modify
  - Compare-&-Exchange
  - `simple-atomic`
### Chapter 3: Memory Ordering


### **note**: 
The physical copy I'm using is written for **Stable Rust v`1.66.0`** (Dec. 2022).  
This repo, as of start, is using **Nightly Rust v`1.86.0-nightly`** (Feb. 2023).
I am uncertain whether the online version is updated relative to the physical copy, but am using both.

## Process vs Thread : cartoon
Processes independent, modulo kernel, objects.  Contain raw program instructions, memory info (registers, heap), and OS element info and access info (e.g. open files, sockets).  Also contain info about self and special relationships (e.g. pid & ppid; user & group id ['nix]; scheduling/priority info).
Threads: execution within process: contain **registers** & **stack** & **instruction pointer** ("program counter") and possibly Thread-Local Storage (TLS).
Conceptually, all stack based operations can be considered purview of threads with "core" stack owned by a thread on process.

Separate Instruction Pointers, stacks, and registers allow threads to perform concurrently and specifically in parallel. Co-reading program is nominally not likely to be a bottleneck.  Shared memory and some unnecessary cache invalidations are more likely points of inefficiency.

shorter summary:*Shared heap; separate stacks and program progress*

## Process 
[Operating System Concepts, 9thed, Chapter 3: Processes](https://www.cs.uic.edu/~jbell/CourseNotes/OperatingSystems/3_Processes.html)
(pdf attached)

### (quasi-deets)
Common across current major OSes: 
A process has it's own memory space, including heap stack and registers.  It also has quite a bit of information related to it place in the OS (e.g. scheduling information), elements governed by OS (e.g. "files"), and some other info on neighboring processes (e.g. parent and IPC related info).

Linux: 
 - process address space:: code, data, heap, stack, memory mappings (shared libraries & mmap)
 - posix threads (`pthreads`) map to kernel threads
 - key system calls: `fork`, `exec`, `wait`, `kill`
 - process state: `running`, `sleeping`, `stopped`, `zombie`
 - scheduling uses: Completely Fair Scheduler (CFS)
 - IPS via `pipes` (+ `named pipes`), `message queues`, `shared memory`, `semaphores`, `sockets`
Mac:
 - Linux +/but:
   - mach kernel layer w/ mach ports for IPC
   - mach messages and posix ipc are both used (mach messages are lower level)
   - memory-mapped files and dyld ("Macho-O dynamic linking") are different
   - Threads uses Grand Central Dispatch (GCD)
   - file system: uses APFS; different permission semantics
   - Threads: uses *both* threads and "Mach tasks"; GCD nominally has work queue optimizations
Windows:
 - vs Linux:
   - processes own their memory, handles, and resources (?)
   - threads do not own resources (?)
   - no `fork`; `CreateProcess` instead, which doesn't duplicate parent's address space
   - nominally no "explicit heap"
   - memory mapped files shared between processes
   - threads can use "fiber-based threading" in addition to kernel management
   - Thread-local storage (TLS) is different from unix
   - (object oriented process model?)
   
**main diffs**: on windows threads require more explicit inheritance of process resources, linux/mac they can be filtered, but inherit by default.  Threads also seem to be treated as a more inherent part of a process in 'nix -- this impacts ability to kill a thread separate from its process as well (though on all three systems this seems to be considered generally a bad choice -- vs allowing graceful end of thread with clean up)
```rust
use std::sync::Arc;
use std::collections::HashMap;

pub struct Process {
        pub pid:      u32,
        pub ppid:     u32,
        pub user_id:  u32,
        pub group_id: u32,
        pub memory_mappings:       Vec<MemoryMapping>,
        pub open_file_descriptors: HashMap<u32, FileDescriptor>,
        pub signal_handlers:       HashMap<u8, SignalHandler>,
        pub threads:               Vec<Thread>,
        pub state:                 ProcessState,
        pub scheduling_info:       SchedulingInfo,
        pub ipc_resources:         IpcResources,
        pub resource_limits:       ResourceLimits,
}

pub struct MemoryMapping {
        pub address_range: (usize, usize),
        pub permissions:   MemoryPermissions,
}
pub struct FileDescriptor {
        pub fd:          u32,
        pub file_path:   String,
        pub permissions: FilePermissions,
}
pub struct SignalHandler {
        pub signal: u8,
        pub action: SignalAction,
}
pub struct Thread {
        pub tid:       u32,
        pub registers: CpuRegisters,
        pub stack_pointer:       usize,
        pub instruction_pointer: usize,
}
pub struct CpuRegisters {
        pub general_purpose: [u64; 16], // Architecture dependent
        pub flags:           u64,
}
pub struct SchedulingInfo {
        pub priority:   i32,
        pub nice_value: i32,
}
pub struct IpcResources {
        pub message_queues:         Vec<u32>,
        pub shared_memory_segments: Vec<u32>,
        pub semaphores:             Vec<u32>,
}
pub struct ResourceLimits {
        pub max_cpu_time: u64,
        pub max_memory:   u64,
}
```
