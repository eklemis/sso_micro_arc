use crate::data::{
    create_user, email_or_username_taken, establish_connection, validate_user_email,
};
use crate::input_validation::{is_valid_email, is_valid_password, is_valid_url};
use crate::models::User;
use bcrypt::{hash, DEFAULT_COST};
use diesel::result::Error as DieselError;
use log::{error, info};

#[derive(Debug)]
pub enum RegisterError {
    InvalidEmail,
    InvalidPassword,
    EmailOrUserNameTaken,
    HashingFailed,
    PictureUrlInvalid,
    DatabaseError(DieselError),
}

pub fn register_user(
    username: &str,
    password: &str,
    email: &str,
    fullname: &str,
    phone_number: &str,
    profile_pic_url: &str,
) -> Result<User, RegisterError> {
    // TODO: implement checking data before save to db logic
    let mut conn = establish_connection();

    if !is_valid_email(email) {
        error!(
            "Registration attempt failed with incorrect email format:{}",
            email
        );
        return Err(RegisterError::InvalidEmail);
    }

    if !is_valid_password(password) {
        error!(
            "Registration attempt failed with incorrect password format:{}",
            password
        );
        return Err(RegisterError::InvalidPassword);
    }

    if email_or_username_taken(&mut conn, username, email) {
        error!(
            "Registration attempt failed with taken username or email: {}",
            username
        );
        return Err(RegisterError::EmailOrUserNameTaken);
    }

    if !is_valid_url(profile_pic_url) {
        error!(
            "Registration attempt failed with incorect photo url:{}",
            profile_pic_url
        );
        return Err(RegisterError::PictureUrlInvalid);
    }

    match hash(password, DEFAULT_COST) {
        Ok(hashed_password) => {
            match create_user(
                &mut conn,
                username,
                &hashed_password,
                email,
                fullname,
                phone_number,
                profile_pic_url,
            ) {
                Ok(user) => {
                    info!("User registration successful for username: {}", username);
                    return Ok(user);
                }
                Err(e) => {
                    error!(
                        "Database error during registration for username: {}",
                        username
                    );
                    return Err(RegisterError::DatabaseError(e));
                }
            }
        }
        Err(_) => {
            error!(
                "Registration attempt failed in hashing password for username {}",
                username
            );
            return Err(RegisterError::HashingFailed);
        }
    }
}
pub fn confirmed_email(user_email: &str) -> bool {
    let mut conn = establish_connection();
    let result = validate_user_email(&mut conn, user_email);
    match result {
        Ok(_) => {
            return true;
        }
        Err(err) => {
            error!("DB Failed update user:{}, {:?}", user_email, err);
            return false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::expression_methods::ExpressionMethods;
    use diesel::query_dsl::QueryDsl;

    use diesel::prelude::*;

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("not_an_email"));
    }
    #[test]
    fn test_is_valid_url() {
        assert!(is_valid_url("https://example.com/profile.jpg"));
        assert!(!is_valid_url("not_a_url"));
        assert!(is_valid_url("")); // Should return true according to your requirement

        // Create a URL that is too long (2084 characters)
        let mut long_url = String::from("https://example.com/");
        for _ in 0..2060 {
            long_url.push('a');
        }
        long_url.push_str(".jpg");

        // This URL is too long, so is_valid_url should return false
        assert!(!is_valid_url(&long_url));
    }
    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password("Abc123@xyz"));
        assert!(!is_valid_password("weakpassword"));
    }
    #[test]
    fn test_username_email_taken() {
        use crate::schema::users::dsl::*;

        let test_username = "testuser_again";
        let test_password = "Test_again@1234";
        let test_email = "test@otherexample.com";
        let test_full_name = "Test User";
        let test_phone_number = "1234567890";
        let test_profile_pic_url = "";

        // Create the initial user
        match register_user(
            test_username,
            test_password,
            test_email,
            test_full_name,
            test_phone_number,
            test_profile_pic_url,
        ) {
            Ok(user) => {
                // If registration is successful, try to register a new user with the same username and email
                match register_user(
                    test_username,
                    test_password,
                    test_email,
                    test_full_name,
                    test_phone_number,
                    test_profile_pic_url,
                ) {
                    Ok(_) => {
                        // This should not succeed, as the username and email are already taken
                        assert!(false, "Expected error, username or email already taken");
                    }
                    Err(e) => match e {
                        RegisterError::EmailOrUserNameTaken => {
                            assert!(true);
                        }
                        _ => {
                            assert!(false, "Expected EmailOrUserNameTaken error, got {:?}", e);
                        }
                    },
                }

                // Cleanup after test
                let mut conn = establish_connection();
                diesel::delete(users.filter(username.eq(test_username)))
                    .execute(&mut conn)
                    .expect("Failed to delete test user");
            }
            Err(e) => {
                // If registration failed, print the error (you might want to assert specific error types)
                println!("Initial registration failed with error: {:?}", e);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_register_user() {
        use crate::schema::users::dsl::*;

        let new_user_name = "testuser";
        let new_password = "Test@1234";
        let new_email = "test@example.com";
        let new_full_name = "Test User";
        let new_phone_number = "1234567890";
        let profile_pic_url = "https://example.com/profile.jpg";

        match register_user(
            new_user_name,
            new_password,
            new_email,
            new_full_name,
            new_phone_number,
            profile_pic_url,
        ) {
            Ok(user) => {
                // If registration is successful, the user should exist in the database.
                assert_eq!(new_user_name, user.username);
                assert_eq!(new_email, user.email);
                assert_eq!(new_full_name, user.fullname);
                assert_eq!(
                    new_phone_number,
                    user.phone_number.unwrap_or_default().as_str()
                );

                // Cleanup after test
                let mut conn = establish_connection();
                diesel::delete(users.filter(username.eq(new_user_name)))
                    .execute(&mut conn)
                    .expect("Failed to delete test user");
            }
            Err(e) => {
                // If registration failed, print the error (you might want to assert specific error types)
                println!("Registration failed with error: {:?}", e);
                assert!(false);
            }
        }
    }
}
