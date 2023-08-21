pub mod session;
use crate::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use dotenvy::dotenv;

use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_user(
    conn: &mut PgConnection,
    username: &str,
    password_hash: &str,
    email: &str,
    fullname: &str,
    phone_number: &str,
    profile_picture_url: &str,
) -> Result<User, DieselError> {
    use crate::schema::users;
    let new_user = NewUser {
        username,
        password_hash,
        email,
        fullname,
        phone_number,
        profile_picture_url,
    };
    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn email_or_username_taken(conn: &mut PgConnection, user_name: &str, em: &str) -> bool {
    use crate::schema::users::dsl::*;
    let count = users
        .filter(username.eq(user_name).or(email.eq(em)))
        .count()
        .get_result::<i64>(conn)
        .expect("Failed to perform count query");
    count > 0
}
pub fn get_user(conn: &mut PgConnection, em: &str) -> Option<User> {
    use crate::schema::users::dsl::*;
    let rows: Vec<User> = users
        .filter(email.eq(em))
        .limit(1)
        .select(User::as_select())
        .load(conn)
        .expect("Error loading user");
    if rows.len() > 0 {
        return Some(User {
            user_id: rows[0].user_id,
            username: rows[0].username.clone(),
            password_hash: rows[0].password_hash.clone(),
            email: rows[0].email.clone(),
            fullname: String::from(""),
            phone_number: None,
            profile_picture_url: None,
            email_validated: rows[0].email_validated,
            phone_number_validated: rows[0].email_validated,
        });
    }
    None
}
pub fn validate_user_email(
    conn: &mut PgConnection,
    user_email: &str,
) -> Result<usize, DieselError> {
    use crate::schema::users::dsl::*;
    diesel::update(users)
        .filter(email.eq(user_email))
        .set(email_validated.eq(true))
        .execute(conn)
}
