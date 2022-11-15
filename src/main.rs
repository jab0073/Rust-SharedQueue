mod process;
mod consumer_thread;
mod producer_thread;

use std::borrow::Borrow;
use std::collections::BinaryHeap;
use std::io::{Read, stdin};
use std::sync::{Arc, Mutex};
use crate::process::Process;
use crate::producer_thread::Producer;


pub fn main() {
    println!("Enter the number of nodes to generate: ");

    let mut input: String = String::new(); // Declaring a new mutable string variable to use as the input buffer
    input.clear(); // Clear the buffer to ensure it is empty
    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer
    let number_of_nodes: usize = input.trim_end_matches('\n').parse().unwrap();

    println!("Enter sleep time in ms for the producer to pause between generation phases:");

    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer

    let producer_sleep_time: i32 = input.trim_end_matches('\n').parse().unwrap();

    println!("Enter number of processes to generate each phase:");

    stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input"); // Read the input from the user storing it in the buffer

    let produce_at_a_time: i32 = input.trim_end_matches('\n').parse().unwrap();

    let mut bh = Arc::new(Mutex::new(BinaryHeap::new()));

    let mut producer = Producer::new(producer_sleep_time, number_of_nodes as i32, produce_at_a_time, bh);

    producer.run();


    /*for id in 1..number_of_nodes + 1 {
        let proc: process::Process = process::process_builder(id as i32); // Creating a new process
        bh.push(proc); // Store the copy in the binary heap
    }*/

    println!("\n!!! Verifying. The heap contains {} elements", bh.lock().unwrap().len()); // Printing verification of binary heap size


    /*println!("\n\n!!! Now, draining the MinHeap, one process at a time...");
    for _ in 0..bh.len() {
        // Iterating through the deque and popping the top
        let process = bh.pop();
        println!(
            "{}",
            process.expect("Process unable to be retrieved").to_string()
        )
    }*/

    println!("Goodbye :)") // Bye Bye :)
}

