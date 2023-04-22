#![allow(dead_code)]
#![allow(unused_variables)]

mod catsec;
use catsec::*;

use std::io;
use std::io::Write;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use md5;
use rpassword::read_password;


fn login() -> String {
    
    println!("CatSec v{}", env!("CARGO_PKG_VERSION"));
    println!("Welcome to CatSec , A password Manager");

    print!("username:");
    io::stdout().flush().ok();
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    print!("password:");
    io::stdout().flush().ok();
    let password = read_password().unwrap();

    let result = verify_login(&username, &password);

    if result == false {
        let mut login = Vec::new();
        let password = md5::compute(&password);
        login.push(username.replace("\n", ""));
        login.push(format!("{:x}", password));
        save(login,"password");
    }
    return username;
}

fn verify_login(username: &str, password: &str) -> bool  {
    let mut result = false;
    let file = File::open(".catsec.key").unwrap();
    let mut lector = BufReader::new(file);
	let mut cdn_buff = String::new();
    lector.read_line(&mut cdn_buff).unwrap();

    if cdn_buff.len() != 0 {
        let parts = cdn_buff.split(",");
        let collect = parts.collect::<Vec<&str>>();
        let pass = collect[0];
        let passw = format!("{:x}", md5::compute(&password));

        if pass == passw {
            result = true;
        }else{
            login();
        }
    }

    return result;
}

fn save(data: Vec<String>, mode: &str) {
    if mode == "password" {
        let mut save_pass = File::create(".catsec.key").unwrap();
        save_pass.write_all(&data[0].as_bytes()).unwrap();
    }else if mode == "database" {
        let mut dat = String::new();
        for d in &data {
            dat.push_str(&d);
        }
        let mut save_data = OpenOptions::new()
                    .write(true)
                    .create(false)
                    .open("catsec.data")
                    .unwrap();
        save_data.(&dat.as_bytes()).unwrap();
    }else{
        panic!("Unimplemented function!");
    }
}

fn help() {
    println!(" add ............ add a new PasswordData.");
    println!(" mod ............ modify a PasswordObject.");
    println!(" del ............ delete a PasswordObject");
    println!(" list ........... list all PasswordObjects");
    println!(" show........... show a PasswordObject");
    println!(" q ............. exit program");
}

fn show_passwords(catsec: &PassStorage) {
    for password in catsec.passwords.iter() {
        println!("Site: {}\nUsername: {}\nPassword: {}\n", password.site, password.username, password.password);
    }
}

fn main() {
    
    let mut catsec = PassStorage { passwords: vec![]};
    let mut keepalive = true;
    let user = login();
    while keepalive == true {

        print!("catsec@{}$ ",user.replace("\n",""));
        io::stdout().flush().ok();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let matches = input.replace("\n","");

        match matches.as_str() {
            "add" => {
                print!("Website:");
                io::stdout().flush().ok();
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                site = site.trim().to_string();

                print!("Username:");
                io::stdout().flush().ok();
                let mut username = String::new();
                io::stdin().read_line(&mut username).unwrap();
                username = username.trim().to_string();
                
                print!("Password:");
                io::stdout().flush().ok();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                password = password.trim().to_string();

                let passw = Password {
                    site,
                    username,
                    password,
                };

                let mut passw2 = passw.clone();
                passw2.site.push_str(",");
                passw2.username.push_str(",");
                passw2.password.push_str(";");
    
                save(vec![passw2.site,
                    passw2.username,
                    passw2.password], "database");
                catsec.add_password(passw);
                
            },
            "mod" => {
                print!("Website of password to modify:");
                io::stdout().flush().ok();
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                site = site.trim().to_string();

                print!("New password:");
                io::stdout().flush().ok();
                let mut password = String::new();
                io::stdin().read_line(&mut password).unwrap();
                password = password.trim().to_string();

                if catsec.update_password(&site, &password) {
                    println!("Password updated successfully!");
                } else {
                    println!("Incorrect data !");
                }

            },
            "del" => {
                print!("Website of password to delete:");
                io::stdout().flush().ok();
                let mut site = String::new();
                io::stdin().read_line(&mut site).unwrap();
                site = site.trim().to_string();

                if catsec.delete_password(&site) {
                    println!("Password delete successfully!");
                } else {
                    println!("Incorrect data !");
                }
            },
            "list" => {
                if catsec.passwords.is_empty() {
                    println!("Empty password list!");
                } else {
                    show_passwords(&catsec);
                }
            },
            "q" => keepalive = false,
            _ => {
                println!("Incorrect operation , try with: ");
                help();
            },
        }
    }
}