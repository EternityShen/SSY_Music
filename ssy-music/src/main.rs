// 依赖过多,同名函数结构体和方法很多,所以我决定全部的调用都用完整路径(当然,有些我不知道在哪的就直接让Lsp补全了)

use ssy_music::ui::app::App;

fn main() {
    init_app();

    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .window(iced::window::Settings {
            icon: load_icon(),
            ..iced::window::Settings::default()
        })
        .run()
        .unwrap();
}

fn load_icon() -> Option<iced::window::Icon> {
    let icon_bytes = include_bytes!("/home/eternity/Music/专辑图片/无法解析:未知.jpeg");

    let image = image::load_from_memory(icon_bytes).ok()?;
    let rgba = image.into_rgba8();
    let (width, height) = rgba.dimensions();
    let rgba_bytes = rgba.into_raw();
    iced::window::icon::from_rgba(rgba_bytes, width, height).ok()
}

fn init_app() {
    let mut user_config_dir = dirs::config_dir().ok_or("找不到系统配置目录").unwrap();

    user_config_dir.push("ssy-music");

    if !user_config_dir.exists() {
        std::fs::create_dir(&user_config_dir).unwrap();
    }

    let taeget_file = user_config_dir.join("config.toml");

    if !taeget_file.exists() {
        let log_path = user_config_dir.join("log.log");

        let load_db_path = user_config_dir.join("music_db.toml");

        std::fs::write(
            &load_db_path,
            r#"[songs.0]
id = 0 # id
title = "说了再见" # 歌名
artist = "周杰伦" # 歌手
album = "跨时代" # 专辑名
path = "/home/eternity/Music/音频文件/说了再见.mp3" # 音频文件路径
image = "/home/eternity/Music/专辑图片/周杰伦-跨时代.jpg" # 专辑图片路径
duration = 282.83 # 时长 s"#,
        )
        .unwrap();

        let log_path_config = format!("log_path=\"{}\" # log文件路径", log_path.to_string_lossy());

        let load_db_config = format!(
            "load_db_path=\"{}\" # 本地数据量路径",
            load_db_path.to_string_lossy()
        );

        std::fs::write(
            &taeget_file,
            format!(
                "{}\n{}\n{}\n{}\n{}",
                log_path_config,
                load_db_config,
                "lyrics_path = \"歌词的目录\" # 歌词的存放目录",
                "play_mode = \"Load\" # 播放模式 Load(本地) Net(网络)",
                "net_link = \"127.0.0.1:3000\" # 网络播放的api链接,只能用配套的server"
            ),
        )
        .unwrap();
    }
}

// use std::sync::Mutex;
//
// use client::{api, player};
// use futures_util::StreamExt;

// let logger_handle = std::sync::Arc::new(Mutex::new(logger::Logger::new("./client.log")));
//
// let api = api::request::MusicClient::new("127.0.0.1:3000", logger_handle);
//
// let player = player::handle::PlayerHandle::default();
//
// let stream = api.fetch_audio_stream(2).await.unwrap();
//
// tokio::pin!(stream);
//
// let mut audio_bytes = Vec::new();
//
// let mut chunk_count = 0;
//
// while let Some(chunk_result) = stream.next().await {
//     match chunk_result {
//         Ok(bytes) => {
//             audio_bytes.extend_from_slice(&bytes);
//             chunk_count += 1;
//             if chunk_count >= 3 {
//                 break;
//             }
//         }
//         Err(_e) => {}
//     }
// }
//
// player.play_form_bytes(audio_bytes);
//
// println!("正在播放...............................................OvO");
//
// loop {
//     std::thread::sleep(std::time::Duration::from_millis(100));
//     if player.is_empty() {
//         println!("播放完毕，退出QvQ");
//         break;
//     }
// }
