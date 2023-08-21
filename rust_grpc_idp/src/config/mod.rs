use dotenvy::dotenv;
use std::env;

pub fn get_env_var(var_name: &str) -> String {
    dotenv().ok();
    env::var(var_name).expect("ACCESS_TOKEN_PRIVATE_KEY must be set in the .env file")
}
