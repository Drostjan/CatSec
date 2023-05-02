use std::io;
use std::io::Write;

use aes::Aes128;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

use rpassword::read_password;
use crate::pass::*;

pub struct Console {
    logged: bool,
    username: String,
    passwords: PassStorage,
    key: Vec<u8>,
}

impl Console {
    pub fn new() -> Console {
        Console {
            logged: false,
            username: String::new(),
            passwords: PassStorage::new(),
            key: Vec::new(),
        }
    }

    pub fn login(&mut self) -> String {
        println!("CatSec v{}", env!("CARGO_PKG_VERSION"));
        println!("Welcome to CatSec , A password Manager");
    
        print!("username:");
        io::stdout().flush().ok();
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();
    
        print!("password:");
        io::stdout().flush().ok();
        let password = read_password().unwrap();

        self.username = username;
        self.logged = true;

        let mut cmd = String::from("catsec@");
        cmd.push_str(&self.username);
        
        return cmd;
    }

    pub fn add(&mut self){

        print!("username:");
        io::stdout().flush().ok();
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();
    
        print!("password:");
        io::stdout().flush().ok();
        let password = read_password().unwrap();

        print!("site:");
        io::stdout().flush().ok();
        let mut site = String::new();
        io::stdin().read_line(&mut site).unwrap();

        let key = GenericArray::from([0u8; 16]);
        self.key = key.to_vec();
        
        let mut block = GenericArray::default();
        block.copy_from_slice(password.as_bytes());
        let cipher = Aes128::new(&key);

        cipher.encrypt_block(&mut block);
        let ciphertext = String::from_utf8(block.to_vec()).unwrap();

        let pass = Password {
            username: username.replace("\n", ""),
            password: ciphertext.to_string(),
            site: site.replace("\n", "")
        };

        self.passwords.add_password(pass);
    }

    pub fn update(&mut self){
        print!("site to modify password:");
        io::stdout().flush().ok();
        let mut site = String::new();
        io::stdin().read_line(&mut site).unwrap();

        print!("new password:");
        io::stdout().flush().ok();
        let new_password = read_password().unwrap();
        
        self.passwords.update_password(&site,&new_password);
    }

    pub fn del(&mut self){
        print!("site to delete password data:");
        io::stdout().flush().ok();
        let mut site = String::new();
        io::stdin().read_line(&mut site).unwrap();

        self.passwords.delete_password(site);
    }

    pub fn list(&self){
        println!("{:#?}",self.passwords);
    }

    pub fn show(&self){
        print!("site to show password data:");
        io::stdout().flush().ok();
        let mut site = String::new();
        io::stdin().read_line(&mut site).unwrap();

        let pass = self.passwords.get_password(site);
        println!("{:#?}",pass);
    }

    pub fn help(&self) {
        println!(" add ............ add a new PasswordData.");
        println!(" mod ............ modify a PasswordObject.");
        println!(" del ............ delete a PasswordObject");
        println!(" list ........... list all PasswordObjects");
        println!(" show........... show a PasswordObject");
        println!(" q ............. exit program");
    }

    pub fn save(&self, filename: &str) {
        self.passwords.save(filename);
    }

    pub fn load(&mut self, filename: &str){
        self.passwords.load(filename);
    }
    
}