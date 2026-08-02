# 🎵 SSY-Music

[![Rust](https://img.shields.io/badge/language-Rust_2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-lightgrey.svg)]()

**SSY-Music** 是一款基于 Rust 开发的轻量级跨平台音乐播放器。
---

## ✨ 架构与技术栈

* **Server (服务端)**：负责本地音乐库管理、数据持久化与媒体流高效切片分发。
  * **异步运行时**：Tokio
  * **数据存储**：轻量级 TOML 结构化文本数据库
* **SSY-Music (客户端)**：极致流畅的现代原生 GUI 交互界面。
  * **GUI 框架**：[Iced](https://github.com/iced-rs/iced) (0.14+)
  * **音频引擎**：[Rodio](https://github.com/rust-audio/rodio)
  * **网络通信**：Reqwest (Async)

---

## 🚀 快速开始

### 1. 服务端配置与启动 (Server)

1. 在 `server/` 目录下准备你的音乐数据库文件（如 `music_db.toml`）并修改 `server/src/main.rs`：

```toml
[songs.0]
id = 0 
title = "说了再见"
artist = "周杰伦"
album = "跨时代"
path = "/path/to/your/music/说了再见.mp3"
image = "/path/to/your/cover/周杰伦-跨时代.jpg"
duration = 282.83

[songs.1]
id = 1
title = "走狗"
artist = "周柏豪"
album = "Beginning"
path = "/path/to/your/music/走狗.mp3"
image = "/path/to/your/cover/周柏豪-Beginning.webp"
duration = 249.35

```

```rust
// 创建数据库(纯废话)
    let data = Arc::new(api::data::Data::load_from_toml(
        "./Test/music_db.toml",
        "/home/eternity/Music/歌词/", // 歌词的路径,歌词文件名是拼接来得
        logger_handle,
    ));
```

2. 启动服务端（默认监听 `127.0.0.1:3000`，如需修改请直接调整 `server/src/main.rs` 中的 `bind` 参数）：

```bash
cargo run -p server --release

```

### 2. 客户端安装与运行 (SSY-Music)

```bash
cargo run -p ssy-music --release

```

> **说明**：首次运行时，客户端会自动在用户目录下生成配置文件和本地数据库文件 `~/.config/ssy-music/config.toml` , `~/.config/ssy-music/music_db.toml` 。


---

## 📡 API 接口说明 (RESTful API)

服务端提供以下简洁高能的 REST 接口：

| HTTP 方法 | Endpoint | 说明 | 示例 / 参数 |
| --- | --- | --- | --- |
| `GET` | `/api/songs` | 获取歌单列表 / 搜索 | `/api/songs?keyword=周杰伦` |
| `GET` | `/api/songs/{id}/stream` | 获取音频流 (Audio Stream) | 用于播放器流式解码播放 |
| `GET` | `/api/songs/{id}/lyrics` | 获取歌词 | 目前客户端支持 `.lrc` 格式解析 |
| `GET` | `/api/songs/{id}/image` | 获取专辑封面图片 | 返回图片二进制流 |
| `GET` | `/api/songs/{id}/data` | 获取特定歌曲元数据 | 返回单曲 JSON 详情 |

---

## 🛠️ 路线图 (Roadmap)

* [x] 基于 TOML 的轻量级 Server 数据库及流媒体分发
* [x] 基于 Iced 的客户端 UI 交互与音频播放
* [x] LRC 歌词解析支持
* [ ] [ ] 播放列表/历史记录持

---

## 📄 开源许可

本项目遵循 [MIT License](https://www.google.com/search?q=LICENSE) 开源协议。
