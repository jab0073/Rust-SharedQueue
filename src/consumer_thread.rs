use std::collections::BinaryHeap;
use std::intrinsics::raw_eq;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::process::Process;

// Struct for the Consumer. It has no members as all relevent info is passed
// directly into its run function
pub(crate) struct Consumer {}

impl Consumer {
    pub fn run(self, thread_id: i32, heap: Arc<Mutex<BinaryHeap<Process>>>) {
        // Printing that the specific thread has started
        println!("Consumer {}: starting...", thread_id);

        // Looping to ensure the function doesn't end with out meeting certain criteria
        loop {
            // Acquiring the lock on the Binary Heap
            let locked = heap.lock();

            // Checking to ensure the lock was actually acquired
            if locked.is_ok() {
                // Getting the actual heap from the MutexGuard
                let mut locked_heap = locked.unwrap();

                // Checking to make sure the heap isn't empty
                if !locked_heap.is_empty() {
                    // Getting a process off the top of the heap (highest priority)
                    let process: Process = locked_heap.pop().expect("Consumer pop error");
                    // Since the lock on the heap won't unlock until the heap is out of scope, I'm
                    // forcing the heap to go out of scope before process execution to ensure the
                    // producer and other consumers can acquire the lock
                    drop(locked_heap);
                    // Executing the process
                    sleep(Duration::from_millis(process.get_sleep() as u64));
                    println!("Consumer {}: {} --- EXECUTED", thread_id, process);
                } else {
                    // If the heap was empty then the Consumer has no more work left to do
                    println!("Consumer {}: Believes queue is empty", thread_id);
                    return;
                }
            } else {
                // This will basically never execute due to the thread being blocked at the
                // "heap.lock()" statement if the lock was not able to be acquired
                println!("Consumer {}: BLOCKED", thread_id);
                continue;
            }
        }
    }
}
