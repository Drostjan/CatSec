use std::io;
use std::io::Write;
use rpassword::read_password;

use crate::pass::*;

pub struct Console {
    logged: bool,
    cmd: String,
    username: String,
    passwords: PassStorage
}

impl Console {
    pub fn new() -> Console {
        Console {
            logged: false,
            cmd: String::from("catsec@"),
            username: String::new(),
            passwords: PassStorage::new()
        }
    }

    pub fn login(&mut self){
        println!("CatSec v{}", env!("CARGO_PKG_VERSION"));
        println!("Welcome to CatSec , A password Manager");
    
        print!("username:");
        io::stdout().flush().ok();
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();
    
        print!("password:");
        io::stdout().flush().ok();
        let password = read_password().unwrap();
    }

    pub fn help(&self) {
        println!(" add ............ add a new PasswordData.");
        println!(" mod ............ modify a PasswordObject.");
        println!(" del ............ delete a PasswordObject");
        println!(" list ........... list all PasswordObjects");
        println!(" show........... show a PasswordObject");
        println!(" q ............. exit program");
    }
}