use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub bind_addr: std::net::SocketAddr,
    pub invite_key: String,
    pub database_url: String,
}

impl Config {
    pub fn get() -> Self {
        let path =  match std::env::args().nth(1) {
            Some(p) => p,
            None => "./config.toml".to_owned(),
        };

        let content = std::fs::read_to_string(path)
            .expect("could not read content of config file (does it exist?).");
        toml::from_str(&content).expect("malformed config, could not parse.")
    }
}
