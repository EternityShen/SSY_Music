use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

/// 每首歌的结构体(存歌的数据用的)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    // 歌曲id
    pub id: u64,
    // 名字
    pub title: String,
    // 歌手
    pub artist: String,
    // 专辑
    pub album: String,
    // 音频文件的url
    pub path: String,
    // 专辑图片的url
    pub image: String,
    // 音乐时长(s)
    pub duration: f64,
}

/// 存歌的HashMap(只是用来解析数据库用的，其他地方没鸟用)
#[derive(Debug, Deserialize)]
pub struct SongsHashMap {
    pub songs: HashMap<String, Song>,
}

/// 存放数据的结构体(没什么好说的，查询什么的都在这里)
#[derive(Clone)]
pub struct Data {
    /// 用id来当key(请求更好写)
    pub songs_by_id: HashMap<u64, Song>,
    /// 歌词存放的路径(懒得写进数据库，也没必要)
    ///     直接拼接歌名和歌手即可
    pub lyrics_dir: PathBuf,
    /// 自己写的log系统
    ///     垃圾一个(凑合用)
    logger_handle: Arc<Mutex<logger::Logger>>,
}

impl Data {
    /// 从toml拿数据，懒得用sql(不想学)
    pub fn load_from_toml(
        file_path: &str,
        lyrics_dir: &str,
        logger_handle: Arc<Mutex<logger::Logger>>,
    ) -> Self {
        // 读
        let result = std::fs::read_to_string(file_path);
        let content = match result {
            Ok(s) => s,
            Err(e) => {
                if let Ok(mut log) = logger_handle.lock() {
                    log.error(format!("无法读取数据库 错误:{}", e));
                }
                panic!();
            }
        };

        // 转换
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
                    lyrics_dir: PathBuf::from(lyrics_dir),
                    logger_handle,
                }
            }
            Err(e) => {
                if let Ok(mut log) = logger_handle.lock() {
                    log.error(format!("无法将toml转换为HashMap 错误:{}", e));
                }
                panic!();
            }
        }
    }

    /// 歌名/专辑/歌手搜索 ，keyword为空时直接返回整个歌曲列表(HashMap内的全部)
    ///     -> Vec<Song>
    pub fn search_songs(&self, keyword: Option<String>) -> Vec<Song> {
        let all_songs = self.songs_by_id.values();

        let mut list: Vec<Song> = if let Some(kw) = keyword {
            let kw_low = kw.to_lowercase();

            if let Ok(mut log) = self.logger_handle.lock() {
                log.info(format!("用户搜索:{}", kw));
            }

            all_songs
                // 搜
                .filter(|song| {
                    song.title.to_lowercase().contains(&kw_low)
                        || song.artist.to_lowercase().contains(&kw_low)
                        || song.album.to_lowercase().contains(&kw_low)
                })
                .cloned()
                .collect()
        } else {
            if let Ok(mut log) = self.logger_handle.lock() {
                log.info("用户访问歌单".to_string());
            }
            all_songs.cloned().collect()
        };

        list.sort_by_key(|s| s.id);

        list
    }

    /// 查找歌曲的 音频文件路径，直接从HashMap里拿
    ///     -> Result<PathBuf, axum::http::StatusCode>
    pub fn find_song_path(&self, id: u64) -> Result<PathBuf, axum::http::StatusCode> {
        if let Some(song) = self.songs_by_id.get(&id) {
            let path = PathBuf::from(&song.path);

            // 检查文件是否存在，虽然没有傻子会将不存在的路径写进数据库
            if path.exists() {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!(
                        "用户访问 歌曲id:{}  路径:{}",
                        id,
                        path.to_string_lossy()
                    ));
                }
                Ok(path)
            } else {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "歌曲 ID: {} 的本地文件不存在，路径: {:?}",
                        id, path
                    ));
                }

                Err(axum::http::StatusCode::NOT_FOUND)
            }
        } else {
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }

    /// 获取歌词 歌词通过歌曲名-歌手.txt 储存，直接用路径拼接即可
    ///     -> Result<String, axum::http::StatusCode>
    pub fn get_lyrics(&self, id: u64) -> Result<String, axum::http::StatusCode> {
        let song = self
            .songs_by_id
            .get(&id)
            .ok_or(axum::http::StatusCode::NOT_FOUND)?;

        let lyric_file_name = format!("{}-{}.txt", song.title, song.artist);

        let lyric_path = self.lyrics_dir.join(lyric_file_name);

        // 依旧检查是否存在(这个是必要的)
        if lyric_path.exists() {
            if let Ok(mut log) = self.logger_handle.lock() {
                log.info(format!(
                    "用户访问 歌词id:{}  路径:{}",
                    id,
                    lyric_path.to_string_lossy()
                ));
            }
            std::fs::read_to_string(&lyric_path).map_err(|e| {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "读取歌词文件失败，歌曲: {}, 错误: {}",
                        song.title, e
                    ));
                }
                axum::http::StatusCode::INTERNAL_SERVER_ERROR
            })
        } else {
            if let Ok(mut log) = self.logger_handle.lock() {
                log.warn(format!(
                    "未找到歌词文件，歌曲: {}, 预期路径: {:?}",
                    song.title, lyric_path
                ));
            }
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }

    /// 获取图片的
    ///     -> Result<PathBuf, axum::http::StatusCode>
    pub fn find_image_path(&self, id: u64) -> Result<PathBuf, axum::http::StatusCode> {
        if let Some(song) = self.songs_by_id.get(&id) {
            let path = PathBuf::from(&song.image);

            // 依旧检查是否存在
            if path.exists() {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!(
                        "用户访问 图片id:{}  路径:{}",
                        id,
                        path.to_string_lossy()
                    ));
                }
                Ok(path)
            } else {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "歌曲 ID: {} 的图片文件不存在，路径: {:?}",
                        id, path
                    ));
                }
                Err(axum::http::StatusCode::NOT_FOUND)
            }
        } else {
            Err(axum::http::StatusCode::NOT_FOUND)
        }
    }

    /// 获取音乐数据
    ///     -> Option<Song>
    pub fn get_song(&self, id: u64) -> Option<Song> {
        if let Some(song) = self.songs_by_id.get(&id) {
            Some(song.clone())
        } else {
            None
        }
    }
}
