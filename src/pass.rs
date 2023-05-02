use std::fs::File;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
    pub username: String,
    pub password: String,
    pub site: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassStorage {
    pub passwords: Vec<Password>,
}

impl PassStorage {
    pub fn new() -> PassStorage {
        PassStorage {
            passwords: Vec::new(),
        }
    }
    pub fn add_password(&mut self, password: Password) {
        self.passwords.push(password);
    }

    pub fn update_password(&mut self, site: &str, new_password: &str) {
        for password in self.passwords.iter_mut() {
            if password.site == site {
                password.password = new_password.to_string();

            }
        }
    }

    pub fn delete_password(&mut self, site: String){
        for i in 0..self.passwords.len() {
            if self.passwords[i].site == site {
                self.passwords.remove(i);
            }
        }
    }

    pub fn get_password(&self, site: String){
        let mut the_password = None;
        for pass in self.passwords.iter() {
            if pass.site == site {
                the_password = Some(pass.clone());
                print!("{}", pass.password);
            }
        }

        if let Some(password) = the_password {
            println!("{:?}", password);
        } else {
            println!("No found site '{}'", site);
        }
    }

    pub fn save(&self, filename: &str) {
        let mut file = File::create(filename).unwrap();
        let serialized = serde_json::to_string(self).unwrap();
        file.write_all(serialized.as_bytes()).unwrap();
    }

    pub fn load(&mut self, filename: &str) {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let deserialized: PassStorage = serde_json::from_str(&contents).unwrap();
        *self = deserialized;
    }

}