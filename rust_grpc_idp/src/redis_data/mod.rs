use crate::config::get_env_var;
extern crate redis;
use redis::{Commands, Connection};

fn get_connetion() -> redis::RedisResult<Connection> {
    let con_str = get_env_var("REDIS_CON_STR");
    let client = redis::Client::open(con_str)?;
    let con = client.get_connection()?;
    return Ok(con);
}
pub fn push_str(key: &str, value: &str, expiry: usize) -> redis::RedisResult<String> {
    println!("Redis push : {}", key);
    let mut con = get_connetion()?;
    let _: () = con.set_ex(key, value, expiry)?;
    con.get(key)
}
pub fn fetch_str(key: &str) -> redis::RedisResult<String> {
    let mut con = get_connetion()?;
    con.get(key)
}
pub fn del_str(key: &str) -> redis::RedisResult<String> {
    let mut con = get_connetion()?;
    con.del(key)
}
