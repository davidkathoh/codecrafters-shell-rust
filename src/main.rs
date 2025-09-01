#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, fs, path::Path,process::Command};


fn main() {
    // Uncomment this block to pass the first stage
    

    // Wait for user input
    let   commands = ["echo","exit","type","pwd","cat"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let mut inputs:Vec<&str> = input.split(' ')
                .map(|s|s.trim_end())
                .collect();
        let cmd_1 = inputs.first().unwrap().trim();   

        match cmd_1 {
            "echo"=>{
                inputs.remove(0);

              let x =  input.split_once(' ').unwrap();
              
             if is_enclosed(x.1){
                let y = x.1.replace("'", "");
                println!("{}",y.trim())
             }else{
                
                let z:Vec<String> = inputs.iter().filter(|s|!s.is_empty()).map(|s|s.to_string()).collect();
              
                println!("{}",z.join(" ").trim())//.join(" ").trim());  
             }
            }
            "exit"=>{
                if inputs.get(1).unwrap() ==&"0" {
                    break;
                }
            }
            "type"=>{
                if commands.contains(inputs.get(1).unwrap()) {
                    println!("{} is a shell builtin", inputs.get(1).unwrap().trim()); 
                }else if env::var("PATH").is_ok() {
                    let path   =  env::var("PATH").unwrap();
    
                    let split = &mut path.split(":");
                    let cmd = inputs.first().unwrap().trim();
                    if let Some(dir) =  split.find(|dir| fs::metadata(format!("{}/{}",dir,cmd)).is_ok()){
                        println!("{cmd} is {dir}/{cmd}")
                    }else {
                        println!("{cmd}: not found"); 
                    }
            }else{
                println!("{}: not found",inputs.get(1).unwrap().trim());
            }
        
        }
            "pwd"=>{
                println!("{}",env::current_dir().unwrap().display())
            }
            "cd"=>{

            let mut path = inputs.get(1).unwrap_or(&"").trim().to_string();
            if path =="~" {
                path = env::var("HOME").unwrap();
            }
            match env::set_current_dir(path) {
                Ok(_) => continue,
                Err(_) => println!("{}: {}: No such file or directory",cmd_1,inputs.get(1).unwrap_or(&"")),
            }
            }
            "cat"=>{

                let mut  new_input = inputs;
                new_input.remove(0);
                new_input.join("");

                println!("{:?}", new_input);
                println!("cat detected")
            }
            _=>{
                if env::var("PATH").is_ok(){
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
        
                }
                }else{
                println!("{}: command not found",input.trim()); 
                }
            }
            
        }
         
        
    }
}

fn is_enclosed(text:&str)-> bool{
  text.contains("'")
}

