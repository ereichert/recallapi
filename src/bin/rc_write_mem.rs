extern crate recall_api;

use std::io::{self, Read};
use recall_api::models::Memory;

fn main() {
    println!("What is your Memory Prompt?\n");
    let mut mem_prompt_buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let _ = handle.read_to_string(&mut mem_prompt_buffer).unwrap();
    let mem_prompt = mem_prompt_buffer.trim();

    println!("What are the Memory Details?");
    let mut mem_details_buffer = String::new();
    let _ = handle.read_to_string(&mut mem_details_buffer).unwrap();
    let mem_details = mem_details_buffer.trim();

    let memory = Memory::new(mem_prompt, mem_details);

    println!("Prompt:\n{}", memory.prompt);
    println!("Details:\n{}", memory.details);
}
