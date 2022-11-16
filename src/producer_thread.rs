use std::collections::BinaryHeap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::process::Process;

// Struct for the Producer. It has no members as all relevant info is passed
// directly into its run function
pub(crate) struct Producer {}

impl Producer {
    pub fn run(
        self,
        slep: u32,
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
            let mut j: u32 = 0;
            loop {
                let locked = heap.lock();
                if locked.is_ok() {
                    let process: Process = Process::build(id as i32);
                    println!("Producer: {}", process);
                    j += 1;

                    let mut locked_heap = locked.unwrap();
                    locked_heap.push(process);
                    drop(locked_heap);
                    if j == phase {
                        sleep(Duration::from_millis(slep as u64));
                        break;
                    }
                    id += 1;
                } else {
                    println!("Producer: BLOCKED");
                    continue;
                }
            }
            total_generations_created += 1;
            sleep(Duration::from_millis(100 as u64));
            id += 1;
            if total_generations_created == generations {
                println!("Producer: All done");
                return;
            }
        }
    }
}
