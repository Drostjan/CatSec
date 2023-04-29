#![allow(dead_code)]
#![allow(unused_variables)]

mod console;
mod pass;
use console::Console;

use std::io;
use std::io::Write;

fn main() {
    
    let catsec = Console::new();
    let mut keepalive = true;
    while keepalive == true {

        print!("catsec@drost ");
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let matches = input.replace("\n","");

        match matches.as_str() {
            "add" => {

            },
            "mod" => {

            },
            "del" => {
            
            },
            "list" => {
               
            },
            "q" => keepalive = false,
            _ => {
                println!("Incorrect operation , try with: ");
                catsec.help();
            },
        }
    }
}