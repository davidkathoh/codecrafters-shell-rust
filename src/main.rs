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

        let mut inputs:Vec<&str> = input.split(' ')
                .map(|s|s.trim_end())
                .collect();
        if input.trim() == "exit 0" {
            break;
        }else if inputs.first().unwrap() == &"echo"{
            inputs.remove(0);
            println!("{}",inputs.join(" ").trim());   
        }else if inputs.first().unwrap() == &"type" {
            inputs.remove(0);
            if inputs.first().unwrap() == &"echo" {
                println!("{} is a shell builtin", inputs.first().unwrap().trim()); 
            }else if inputs.first().unwrap() == &"exit" {
                println!("{} is a shell builtin", inputs.first().unwrap().trim());
            }else if inputs.first().unwrap() == &"type" {
                println!("{} is a shell builtin", inputs.first().unwrap().trim());
            }
            else  {
                 println!("{}: not found",inputs.first().unwrap().trim());
            }
            
        }
        else{
        println!("{}: command not found",input.trim()); 
        }
        
    }
   
}
