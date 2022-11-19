/*
Tested and developed using:

IDE:

IntelliJ IDEA 2022.2.3 (Ultimate Edition)
Build #IU-222.4345.14, built on October 5, 2022
Runtime version: 17.0.4.1+7-b469.62 aarch64
macOS 13.0
Non-Bundled Plugins:
    org.rust.lang (0.4.180.4932-222)

System:

Model Name:	                MacBook Pro
Model Identifier:	        MacBookPro17,1
Model Number:	            MYD92LL/A
Chip:	                    Apple M1
Total Number of Cores:	    8 (4 performance and 4 efficiency)
Memory:	                    8 GB
*/

use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use crate::process::Process;

// Struct for the Producer. It has no members as all relevant info is passed
// directly into its run function
pub(crate) struct Producer {}

impl Producer {
    pub fn run(
        self,
        sleep_time: u32,
        generations: u32,
        phase: u32,
        heap: Arc<Mutex<BinaryHeap<Process>>>,
    ) {
        println!("Producer: starting...");
        // Variable used for setting the new processes id
        let mut id = 0;
        // Used to keep track of how many generations have been done
        let mut total_generations_created: u32 = 0;
        loop {
            // Used to keep track of how many processes have been created this phase
            let mut j: u32 = 0;
            loop {
                // Attempting to acquire the lock on the heap
                let locked = heap.lock();

                // If the lock was acquired
                if locked.is_ok() {
                    // Create a new process
                    let process: Process = Process::build(id as i32);
                    println!("Producer: {}", process);
                    j += 1;

                    // Get the heap
                    let mut locked_heap = locked.unwrap();

                    // Push the new process onto the heap
                    locked_heap.push(process);

                    // Since the lock on the heap won't unlock until the heap is out of scope, I'm
                    // forcing the heap to go out of scope to ensure the consumers can acquire the lock
                    drop(locked_heap);
                    // If the number of process to generate for this cycle is met then sleep and
                    // this break from the inner loop
                    if j == phase {
                        sleep(Duration::from_millis(sleep_time as u64));
                        break;
                    }
                    // Increment the id
                    id += 1;
                } else {
                    println!("Producer: BLOCKED");
                    continue;
                }
            }
            // Increment the total created generations
            total_generations_created += 1;

            // Increment the id
            id += 1;

            // If the number of generated phases have been met then the producer has no more work
            // left to do so it returns from this function
            if total_generations_created == generations {
                println!("Producer: All done");
                return;
            }
        }
    }
}
