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
