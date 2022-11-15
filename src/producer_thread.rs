use std::borrow::Borrow;
use std::collections::BinaryHeap;
use std::ptr::null;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{JoinHandle, sleep};
use std::time::Duration;

use crate::process::Process;

pub(crate) struct Producer {
    thread: Option<JoinHandle<Process>>,
    sleep: i32,
    total_to_create: i32,
    phase: i32,
    total_been_created: i32,
    heap: Arc<Mutex<BinaryHeap<Process>>>,
}

impl Producer {
    pub fn new(sleep_time: i32, total: i32, per_phase: i32, bh: Arc<Mutex<BinaryHeap<Process>>>) -> Self {
        Self {
            thread: None,
            sleep: sleep_time.clone(),
            total_to_create: total.clone(),
            phase: per_phase.clone(),
            total_been_created: 0,
            heap: Arc::clone(&bh),
        }
    }

    pub fn run(mut self) {
        self.thread = Some(thread::spawn(move || {
            let mut i = 0;
            loop {
                let mut j = 0;
                loop {
                    let process: Process = Process::build(i + j);
                    println!("{}", process);
                    j += 1;
                    self.total_been_created += 1;
                    self.heap.lock().unwrap().push(process);
                    if j > self.phase {
                        sleep(Duration::from_millis(self.sleep as u64));
                        break;
                    }
                }
                sleep(Duration::from_millis(100 as u64));
                i += 1;
            }
        }
        )
        );
    }

    pub fn stop(&mut self) -> Option<thread::Result<Process>> {
        if let Some(handle) = self.thread.take() {
            Some(handle.join())
        } else {
            None
        }
    }
}