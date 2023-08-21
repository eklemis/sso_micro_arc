use crate::schema::users;
use diesel;
use diesel::prelude::*;

//Viewable sesion
#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    pub user_id: i32,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub last_accessed: Option<chrono::NaiveDateTime>,
    pub expires_at: Option<chrono::NaiveDateTime>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
}

//Viewable user
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Debug)]
pub struct User {
    pub user_id: i32,
    pub username: String,
    pub password_hash: String,
    pub email: String,
    pub fullname: String,
    pub phone_number: Option<String>,
    pub profile_picture_url: Option<String>,
    pub email_validated: Option<bool>,
    pub phone_number_validated: Option<bool>,
}

//Writable user
#[derive(Insertable)]
#[diesel(table_name = users)]
#[derive(Debug)]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password_hash: &'a str,
    pub email: &'a str,
    pub fullname: &'a str,
    pub phone_number: &'a str,
    pub profile_picture_url: &'a str,
}
