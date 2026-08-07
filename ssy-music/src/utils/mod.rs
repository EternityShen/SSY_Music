use std::path::PathBuf;

pub fn get_user_config_dir_path() -> PathBuf {
    let mut user_config_dir = dirs::config_dir().ok_or("找不到系统配置目录").unwrap();

    user_config_dir.push("ssy-music");

    user_config_dir
}
