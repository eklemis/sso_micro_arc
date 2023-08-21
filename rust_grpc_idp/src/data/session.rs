use crate::general::generate_unique_string;
use crate::models::Session;
use chrono::Duration;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;

pub fn store_session(
    conn: &mut PgConnection,
    user_id: i32,
    user_ip: &str,
    agen: &str,
) -> Result<Session, DieselError> {
    use crate::schema::sessions;
    let now = chrono::Utc::now().naive_utc();
    let expires_at = now + Duration::minutes(60);

    let new_session = Session {
        session_id: generate_unique_string(),
        user_id,
        created_at: Some(now),
        last_accessed: Some(now),
        expires_at: Some(expires_at),
        ip_address: Some(String::from(user_ip)),
        user_agent: Some(String::from(agen)),
    };
    diesel::insert_into(sessions::table)
        .values(new_session)
        .returning(Session::as_returning())
        .get_result(conn)
}
pub fn get_session(conn: &mut PgConnection, sid: &str) -> Option<Session> {
    use crate::schema::sessions::dsl::*;

    let rows: Vec<Session> = sessions
        .filter(session_id.eq(sid))
        .limit(1)
        .select(Session::as_select())
        .load(conn)
        .expect("Error loading user");
    if rows.len() > 0 {
        let now = chrono::Utc::now().naive_utc();
        if let Some(time) = rows[0].expires_at {
            if time > now {
                println!("Session exist: {:?}", &rows[0]);
                return Some(Session {
                    session_id: rows[0].session_id.clone(),
                    user_id: rows[0].user_id,
                    created_at: rows[0].created_at,
                    last_accessed: rows[0].last_accessed,
                    expires_at: rows[0].expires_at,
                    ip_address: rows[0].ip_address.clone(),
                    user_agent: rows[0].user_agent.clone(),
                });
            }
        }
    }
    None
}
pub fn update_last_access(conn: &mut PgConnection, sid: &str) -> bool {
    use crate::schema::sessions::dsl::*;
    let now = chrono::Utc::now().naive_utc();
    let res = diesel::update(sessions)
        .filter(session_id.eq(sid))
        .set(last_accessed.eq(now))
        .execute(conn);
    match res {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
pub fn make_expired(conn: &mut PgConnection, sid: &str) -> bool {
    use crate::schema::sessions::dsl::*;
    let now = chrono::Utc::now().naive_utc();
    let res = diesel::update(sessions)
        .filter(session_id.eq(sid))
        .set(expires_at.eq(now))
        .execute(conn);
    match res {
        Ok(_) => return true,
        Err(_) => return false,
    }
}
