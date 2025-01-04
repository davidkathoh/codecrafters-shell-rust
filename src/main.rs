#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    

    // Wait for user input
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut inputs:Vec<&str> = input.split(' ').collect();
        if input.trim() == "exit 0" {
            break;
        }else if inputs.first().unwrap() == &"echo"{
            inputs.remove(0);
            println!("{}",inputs.join(" ").trim());   
        }else{
        println!("{}: command not found",input.trim()); 
        }
        
    }
   
}
