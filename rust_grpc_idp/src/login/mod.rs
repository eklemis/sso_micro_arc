use crate::data::{get_user, session::store_session};
use crate::input_validation::is_valid_email;
use crate::models::User;
use diesel::pg::PgConnection;
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum LoginError {
    InvalidEmail,
    EmptyPassword,
    EmptyEmail,
    AccountNotActivated,
    IncorrectCredentials,
    DatabaseError(DieselError),
}
pub fn create_session(
    conn: &mut PgConnection,
    user_id: i32,
    user_ip: &str,
    agen: &str,
) -> Result<String, LoginError> {
    match store_session(conn, user_id, user_ip, agen) {
        Ok(stored_session) => {
            return Ok(stored_session.session_id);
        }
        Err(err) => {
            return Err(LoginError::DatabaseError(err));
        }
    }
}
fn validate_credential(
    conn: &mut PgConnection,
    email: &str,
    password: &str,
) -> Result<User, LoginError> {
    if password.trim() == "" {
        return Err(LoginError::EmptyPassword);
    }
    if email.trim() == "" {
        return Err(LoginError::EmptyEmail);
    }
    if !is_valid_email(email) {
        return Err(LoginError::InvalidEmail);
    }
    if let Some(user) = get_user(conn, email) {
        if let Some(em_validated) = user.email_validated {
            if em_validated {
                if let Ok(res) = bcrypt::verify(password, &user.password_hash) {
                    if res {
                        info!("User {:?} logged in!", user.email);
                        return Ok(user);
                    }
                }
            } else {
                return Err(LoginError::AccountNotActivated);
            }
        }
    }
    return Err(LoginError::IncorrectCredentials);
}
pub fn authenticate(
    conn: &mut PgConnection,
    email: &str,
    password: &str,
) -> Result<User, LoginError> {
    validate_credential(conn, email, password)
}
