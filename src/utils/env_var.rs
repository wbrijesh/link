use dotenvy::dotenv;
use std::env;

pub fn load_env(name: &str) -> Result<String, String> {
    dotenv().ok();
    match env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => Err(format!("{} not found in .env file", name)),
    }
}
