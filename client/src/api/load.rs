use std::{collections::HashMap, path::PathBuf};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SongsHashMap {
    pub songs: HashMap<String, super::data::Song>,
}

pub struct LoadDate {
    pub songs_by_id: HashMap<u64, super::data::Song>,
    db_path: String,
    pub lyrics_dir: PathBuf,
}

impl LoadDate {
    pub fn load_data_from_toml(db_path: String, lyrics_dir: String) -> Self {
        let result = std::fs::read_to_string(&db_path);
        let content = match result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("错误:{}", e);
                panic!();
            }
        };

        let result: Result<SongsHashMap, toml::de::Error> = toml::from_str(&content);

        match result {
            Ok(s_h_m) => {
                let mut songs_by_id = HashMap::new();

                // 转换成用id为key
                for (_, song) in s_h_m.songs {
                    songs_by_id.insert(song.id, song);
                }

                Self {
                    songs_by_id,
                    db_path,
                    lyrics_dir: PathBuf::from(lyrics_dir),
                }
            }
            Err(e) => {
                eprintln!("错误:{}", e);
                panic!();
            }
        }
    }

    pub fn re_load(&mut self) {
        let result = std::fs::read_to_string(&self.db_path);
        let content = match result {
            Ok(s) => s,
            Err(e) => {
                eprintln!("错误:{}", e);
                panic!();
            }
        };

        let result: Result<SongsHashMap, toml::de::Error> = toml::from_str(&content);

        match result {
            Ok(s_h_m) => {
                let mut songs_by_id = HashMap::new();

                // 转换成用id为key
                for (_, song) in s_h_m.songs {
                    songs_by_id.insert(song.id, song);
                }

                self.songs_by_id = songs_by_id;
            }
            Err(e) => {
                eprintln!("错误:{}", e);
                panic!();
            }
        }
    }

    pub fn get_image_path(&self, id: u64) -> Option<String> {
        let option = self.songs_by_id.get(&id);
        option.map(|song| song.image.clone())
    }

    pub fn get_audio_path(&self, id: u64) -> Option<String> {
        let option = self.songs_by_id.get(&id);
        option.map(|song| song.path.clone())
    }

    pub fn get_song_data(&self, id: u64) -> Option<super::data::Song> {
        let option = self.songs_by_id.get(&id);
        option.cloned()
    }

    pub fn get_all_song_data(&self) -> Vec<super::data::Song> {
        let songs: Vec<super::data::Song> = self.songs_by_id.values().cloned().collect();

        songs
    }
}
