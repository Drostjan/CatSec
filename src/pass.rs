#![allow(unused_imports)]

#[derive(Debug, Clone)]
pub struct Password {
    pub username: String,
    pub password: String,
    pub site: String,
}

#[derive(Debug, Clone)]
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

    pub fn get_password(&self, site: String) -> Option<Password>{
        for pass in self.passwords.iter() {
            if pass.site == site {
                return Some(pass.clone())
            }
        }
        return None;
    }

    
}
