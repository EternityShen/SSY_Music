use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_path: String,
    pub load_db_path: String,
    pub lyrics_path: String,
    pub play_mode: String,
    pub net_link: String,
}

impl Config {
    pub fn new() -> Self {
        let content =
            std::fs::read_to_string("/home/eternity/Work/Rust/bin/SSY-Music/Test/config.toml")
                .unwrap();

        toml::from_str(&content).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
