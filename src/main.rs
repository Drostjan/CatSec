#![allow(dead_code)]
#![allow(unused_variables)]

mod console;
mod pass;
use console::Console;

use std::io;
use std::io::Write;

fn main() {
    let mut catsec = Console::new();
    let mut keepalive = true;
    let cmd = catsec.login();

    while keepalive == true {

        print!("{}", cmd.replace("\n"," "));
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let matches = input.replace("\n","");

        match matches.as_str() {
            "add" => {
                catsec.add()
            },
            "mod" => {
                catsec.update()
            },
            "del" => {
                catsec.del()
            },
            "list" => {
                catsec.list()
            },
            "show" => {
                catsec.show()
            },
            "q" => keepalive = false,
            _ => {
                println!("Incorrect operation , try with: ");
                catsec.help();
            },
        }
        
    }
}