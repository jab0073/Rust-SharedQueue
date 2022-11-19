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
use std::io::stdin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use crate::consumer_thread::Consumer;
use crate::producer_thread::Producer;

mod consumer_thread;
mod process;
mod producer_thread;

pub fn main() {
    // Getting the number of phases that will be generated
    println!("Enter the number of generations: ");
    let mut input: String = String::new(); // Declaring a new mutable string variable to use as the input buffer
    input.clear(); // Clear the buffer to ensure it is empty
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer
    let number_of_generations: u32 = input.trim_end_matches('\n').parse().unwrap();

    // Getting the amount of time to sleep between generating each phase
    println!("Enter sleep time in ms for the producer to pause between generation phases:");
    input.clear();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer

    let producer_sleep_time: u32 = input.trim_end_matches('\n').parse().unwrap();

    // Getting the number of processes to generate each phase
    println!("Enter number of processes to generate each phase:");
    input.clear();
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer
    let produce_at_a_time: u32 = input.trim_end_matches('\n').parse().unwrap();

    // Creating a Binary Heap which is wrapped in an Mutex and then a subsequent wrap in an Arc
    // This will allow for operating on the binary heap within multiple threads synchronously
    let bh = Arc::new(Mutex::new(BinaryHeap::new()));

    // Creating the producer
    let producer = Producer {};
    // The producer gets a clone of a reference to the binary heap to be able to operate on it
    let pbh = Arc::clone(&bh);
    // Spawning the producer thread which will call the producers run function.
    let producer_handle = thread::spawn(move || {
        producer.run(
            producer_sleep_time.clone(),
            (number_of_generations as u32).clone(),
            produce_at_a_time.clone(),
            pbh,
        );
        println!("Producer has completed.");
    });

    // Setting up a sleep for the main thread so that the producer has time to fill the binary heap
    // a bit before the consumers start doing their thing
    let wait_time = 500 as u64;
    println!(
        "Waiting for {} milliseconds before starting consumers",
        wait_time
    );
    sleep(Duration::from_millis(wait_time));

    // Creating the first of the consumers
    let consumer1 = Consumer {};

    // Creating the first consumers thread which will call it's run function
    let c1bh = Arc::clone(&bh);
    let consumer1_handle = thread::spawn(move || {
        consumer1.run(1, c1bh);
    });

    // Creating the first of the consumers
    let consumer2 = Consumer {};

    // Creating the second consumers thread which will call it's run function
    let c2bh = Arc::clone(&bh);
    let consumer2_handle = thread::spawn(move || {
        consumer2.run(2, c2bh);
    });

    // Joining the main thread to the three spawned threads to ensure their completion before the
    // main function ends
    producer_handle.join().expect("Producer Join Error");

    consumer1_handle.join().expect("Consumer 1 Join Error");

    consumer2_handle.join().expect("Consumer 2 Join Error");

    // Verifying that the binary heap is empty before the main function ends
    println!(
        "\n!!! Verifying. The heap contains {} elements",
        bh.lock().unwrap().len()
    );

    println!("Goodbye :)") // Bye Bye :)
}
