extern crate todo_swamp;

use std::io;
use std::io::prelude::*;
use todo_swamp::*;

pub fn main() {
    let mut tl: TodoList = TodoList::new();

    let stdin = io::stdin();
    let mut handle = stdin.lock();

    // Read the first line to get the number of input lines to process
    let mut first_line = String::new();
    handle.read_line(&mut first_line).expect("Failed to read line");

    // Parse the first input as a number (and continue processing only that many lines)
    let num_inputs: usize = first_line.trim().parse().expect("Expected a number");

    // Loop only for the number of inputs specified by num_inputs
    for _ in 0..num_inputs {
        let mut line = String::new();
        handle.read_line(&mut line).expect("Failed to read line");

        if !line.trim().is_empty() {
            runner::run_line(&line.trim(), &mut tl);
        }
    }
}
