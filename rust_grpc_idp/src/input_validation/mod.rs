use regex::Regex;
use url::Url;

pub fn is_valid_email(email: &str) -> bool {
    let email_regex = Regex::new(r"(?i)^[A-Z0-9._%+-]+@[A-Z0-9.-]+\.[A-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
pub fn is_valid_password(password: &str) -> bool {
    //Given a password, the task is to validate the password with the help of Regular Expression. A password is considered valid if all the following constraints are satisfied:
    let has_lowercase = Regex::new(r"[a-z]").unwrap().is_match(password);
    let has_uppercase = Regex::new(r"[A-Z]").unwrap().is_match(password);
    let has_digit = Regex::new(r"[0-9]").unwrap().is_match(password);
    let has_special = Regex::new(r"[@#$%^&+=()-]").unwrap().is_match(password);
    let has_no_whitespace = Regex::new(r"\s").unwrap().find(password).is_none();
    let has_correct_length = password.len() >= 8 && password.len() <= 20;
    has_lowercase
        && has_uppercase
        && has_digit
        && has_special
        && has_no_whitespace
        && has_correct_length
}
pub fn is_valid_url(url: &str) -> bool {
    if url.is_empty() {
        return true;
    }
    let urlp = Url::parse(url);
    match urlp {
        Ok(str_url) => str_url.as_str().len() <= 2083,
        Err(_) => false,
    }
}
