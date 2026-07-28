use std::sync::{Arc, Mutex};

use iced::advanced::graphics::core::Bytes;

use crate::api::data::Song;

/// 这玩意主要作用是为了调用不会乱和Log，存这个url是次要的
pub struct MusicClient {
    url: String,
    logger_handle: Arc<Mutex<logger::Logger>>,
}

impl MusicClient {
    pub fn new(ip: &str, logger_handle: Arc<Mutex<logger::Logger>>) -> Self {
        Self {
            url: format!("http://{}", ip),
            logger_handle,
        }
    }
    /// 获取歌曲列表/搜索
    ///     -> Option<Vec<Song>>
    pub async fn fetch_songs(&self, keyword: Option<String>) -> Option<Vec<Song>> {
        let mut url = format!("{}/api/songs", self.url);

        if let Some(kw) = keyword.clone() {
            url = format!("{}?keyword={}", url, kw);
        }

        // 发起网络请求
        let result = reqwest::get(&url).await;
        let response = match result {
            Ok(re) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("网络请求成功 URL: {}", url));
                };
                re
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "网络请求失败，无法连接到服务器。URL: {}, 错误详情: {}",
                        url, e
                    ));
                };

                return None;
            }
        };

        // Json数据转换
        let result = response.json::<Vec<Song>>().await;
        match result {
            Ok(songs) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!(
                        "成功获取歌曲列表。搜索关键字: {:?}, 共找到 {} 首歌曲",
                        keyword,
                        songs.len()
                    ));
                };

                Some(songs)
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.warn(format!(
                        "服务器响应成功，但 JSON 数据解析失败。错误详情: {}",
                        e
                    ));
                };
                None
            }
        }
    }

    /// 获取音乐数据
    ///     -> Option<Song>
    pub async fn fetch_song_data(&self, id: u64) -> Option<Song> {
        let url = format!("{}/api/songs/{}/data", self.url, id);

        // 发起网络请求
        let result = reqwest::get(&url).await;
        let response = match result {
            Ok(re) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("网络请求成功 URL: {}", url));
                };
                re
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "网络请求失败，无法连接到服务器。URL: {}, 错误详情: {}",
                        url, e
                    ));
                };

                return None;
            }
        };

        // Json数据转换
        let result = response.json::<Song>().await;
        match result {
            Ok(song) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("成功获取音乐数据。ID: {}", id));
                }
                Some(song)
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.warn(format!("服务器响应成功，但Json解析失败。错误详情: {}", e));
                };
                None
            }
        }
    }

    /// 获取歌词
    ///     -> Option<String>
    pub async fn fetch_lyrics(&self, id: u64) -> Option<String> {
        let url = format!("{}/api/songs/{}/lyrics", self.url, id);

        // 发起网络请求
        let result = reqwest::get(&url).await;
        let response = match result {
            Ok(re) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("网络请求成功 URL: {}", url));
                };
                re
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!(
                        "网络请求失败，无法连接到服务器。URL: {}, 错误详情: {}",
                        url, e
                    ));
                };

                return None;
            }
        };

        // 转换为文本
        let result = response.text().await;
        match result {
            Ok(str) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("成功获取歌词 ID: {}", id,));
                };

                Some(str)
            }

            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.warn(format!("服务器响应成功，但文本解析失败。错误详情: {}", e));
                };
                None
            }
        }
    }

    /// 获取音频流
    ///     -> Result<impl futures_util::Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error>
    pub async fn fetch_audio_stream(
        &self,
        id: u64,
    ) -> Result<impl futures_util::Stream<Item = Result<Bytes, reqwest::Error>>, reqwest::Error>
    {
        let url = format!("{}/api/songs/{}/stream", self.url, id);

        // 发起网络请求
        let result = reqwest::get(&url).await;
        let response = match result {
            Ok(res) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!("成功建立音频流连接，准备读取数据。ID: {}", id));
                }
                res
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!("请求音频流失败。URL: {}, 错误: {}", url, e));
                }
                return Err(e);
            }
        };

        // 返回流
        Ok(response.bytes_stream())
    }

    /// 获取图片数据
    ///     -> Option<Vec<u8>>
    pub async fn get_image(&self, id: u64) -> Option<Vec<u8>> {
        let url = format!("{}/api/songs/{}/image", self.url, id);

        // 发起网络请求
        let result = reqwest::get(&url).await;

        let response = match result {
            Ok(res) => res,
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!("请求歌曲封面失败。URL: {}, 错误: {}", url, e));
                }
                return None;
            }
        };

        // 检查是否OK
        if !response.status().is_success() {
            if let Ok(mut log) = self.logger_handle.lock() {
                log.warn(format!("获取图片返回错误状态码: {}", response.status()));
            }
            return None;
        }

        // 转换
        match response.bytes().await {
            Ok(bytes) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.info(format!(
                        "成功下载歌曲封面。ID: {}, 大小: {} 字节",
                        id,
                        bytes.len()
                    ));
                }
                Some(bytes.to_vec())
            }
            Err(e) => {
                if let Ok(mut log) = self.logger_handle.lock() {
                    log.error(format!("解析图片字节流失败: {}", e));
                }
                None
            }
        }
    }
}
