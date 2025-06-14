use std::env;

pub struct Config {
    pub bind_addr: String,
}

impl Config {
    pub fn load_from_env() -> Self {
        Self {
            bind_addr: env::var("BIND_ADDR").expect("BIND_ADDR environment variable is not set"),
        }
    }
}
