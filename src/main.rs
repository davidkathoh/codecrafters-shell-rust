#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, path::Path,process::Command};


fn main() {
    // Uncomment this block to pass the first stage
    

    // Wait for user input
    let commands = ["echo","exit","type","pwd"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut inputs:Vec<&str> = input.split(' ')
                .map(|s|s.trim_end())
                .collect();
        let cmd_1 = inputs.first().unwrap();   
        if input.trim() == "exit 0" {
            break;
        }else if cmd_1 == &"echo"{
             inputs.remove(0);
             println!("{}",inputs.join(" ").trim());   
        }else if cmd_1 == &"type" {
            inputs.remove(0);
            if commands.contains(inputs.first().unwrap()) {
                println!("{} is a shell builtin", inputs.first().unwrap().trim()); 
            }else if env::var("PATH").is_ok() {
                let path   =  env::var("PATH").unwrap();

                let split = &mut path.split(":");
                let cmd = inputs.first().unwrap().trim();
                if let Some(dir) =  split.find(|dir| fs::metadata(format!("{}/{}",dir,cmd)).is_ok()){
                    println!("{cmd} is {dir}/{cmd}")
                }else {
                    println!("{cmd}: not found"); 
                }
               
            }
            else  {
                 println!("{}: not found",inputs.first().unwrap().trim());
            }
            
        } else if cmd_1 == &"pwd" {
            println!("{}",env::current_dir().unwrap().display())
        }else if cmd_1 == &"cd"  {

            match env::set_current_dir(Path::new(inputs.get(1).unwrap_or(&""))) {
                Ok(_) => continue,
                Err(_) => println!("{}: {}: No such file or directory",cmd_1,inputs.get(1).unwrap_or(&"")),
            }
        }
        else if env::var("PATH").is_ok(){
            let path   =  env::var("PATH").unwrap();
            let split = &mut path.split(":");
        let cmd = inputs.first().unwrap().trim();
       
        
       
        if let Some(dir) =  split.find(|dir| fs::metadata(format!("{}/{}",dir,cmd)).is_ok()){
            let arg = inputs.get(1).unwrap_or(&"").trim();
            let path =  format!("{}/{}", dir, cmd);
            let output =    Command::new(path)
            .arg(
            arg
            )
            .spawn();

        }else{
            println!("{}: not found",cmd);
        }
           

        }
        else{
        println!("{}: command not found",input.trim()); 
        }
        
    }
   
}
