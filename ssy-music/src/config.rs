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
        let mut user_config_dir = dirs::config_dir().ok_or("找不到系统配置目录").unwrap();
        user_config_dir.push("ssy-music");
        let taeget_file = user_config_dir.join("config.toml");
        let content = std::fs::read_to_string(taeget_file).unwrap();

        toml::from_str(&content).unwrap()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
