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

    pub fn update_password(&mut self, site: &str, new_password: &str) -> bool {
        for password in self.passwords.iter_mut() {
            if password.site == site {
                password.password = new_password.to_string();
                return true;
            }
        }
        false
    }

    pub fn delete_password(&mut self, site: &str) -> bool {
        if let Some(index) = self.passwords.iter().position(|p| p.site == site) {
            self.passwords.remove(index);
            true
        } else {
            false
        }
    }

    
}
