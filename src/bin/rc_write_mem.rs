use std::io::{self, Read};

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

    println!("Prompt:\n{}", mem_prompt);
    println!("Details:\n{}", mem_details);
}
