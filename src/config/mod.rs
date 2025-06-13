use std::env;

pub struct Config {
    pub end_point: String,
    pub bind_addr: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        Self {
            end_point: env::var("END_POINT").expect("END_POINT environment variable is not set"),
            bind_addr: env::var("BIND_ADDR").expect("BIND_ADDR environment variable is not set"),
        }
    }
}
