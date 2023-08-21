use crate::general::generate_unique_string;
use crate::redis_data;

pub struct AuthCode {
    code: String,
}
impl AuthCode {
    pub fn new(user_identity: &str) -> AuthCode {
        let key = generate_unique_string();
        let _ = redis_data::push_str(&key, user_identity, 120).unwrap_or(String::from(""));

        AuthCode { code: key }
    }
    pub fn from(str_key: &str) -> AuthCode {
        AuthCode {
            code: String::from(str_key),
        }
    }
    pub fn remove(&mut self) -> bool {
        if let Ok(_) = redis_data::del_str(&self.code) {
            self.code = String::from("");
            return true;
        }
        false
    }
    pub fn get_code(&self) -> String {
        self.code.clone()
    }
    pub fn is_valid(&self) -> bool {
        if let Ok(rec) = redis_data::fetch_str(&self.code) {
            println!("Received auth code exist for user {}:", rec);
            return true;
        }
        false
    }
    pub fn get_email(&self) -> String {
        if let Ok(rec) = redis_data::fetch_str(&self.code) {
            return rec;
        }
        String::from("")
    }
}
