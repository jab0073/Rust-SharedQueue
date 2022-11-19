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

use std::cmp::Ordering;
use std::fmt;

use rand::distributions::{Distribution, Uniform};

// Defining the Process structure
#[derive(Clone)]
pub(crate) struct Process {
    process_id: i32,
    priority: i16,
    sleep: i16,
    description: String,
}

impl Process {
    pub fn get_sleep(&self) -> i16 {
        self.sleep
    }

    // Process builder function
    pub fn build(id: i32) -> Process {
        let mut rng = rand::thread_rng();
        let priority = Uniform::from(0..101); // Random range for priority
        let sleep = Uniform::from(100..2001); // Random range for sleep
        let proc: Process = Process {
            process_id: id,
            priority: priority.sample(&mut rng), // Getting a value from the range
            sleep: sleep.sample(&mut rng),       // Getting a value from the range
            description: format!("Process with ID of {}", id),
        };
        return proc;
    }
}

// Equal function for a Process
impl Eq for Process {}

// Partial Equal function for a Process
impl PartialEq<Self> for Process {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority // If the priorities are equal than the Processes are equal
    }
}

//Partial Ordering for a Process
impl PartialOrd<Self> for Process {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    } // Delegates to the Ord
}

// Ordering for a Process
impl Ord for Process {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    } // Comparing by priority
}

// Display format for a Process
impl fmt::Display for Process {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "PID: {:<10}PRIORITY: {:<10}SLEEP: {:<10}DESCRIPTION: {:<25}",
            self.process_id, self.priority, self.sleep, self.description
        ) // Format for when printing a Process when using the to_string method
    }
}
