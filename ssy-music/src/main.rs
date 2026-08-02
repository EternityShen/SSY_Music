// 依赖过多,同名函数结构体和方法很多,所以我决定全部的调用都用完整路径(当然,有些我不知道在哪的就直接让Lsp补全了)

use ssy_music::ui::app::App;

fn main() {
    iced::application(App::default, App::update, App::view)
        .subscription(App::subscription)
        .run()
        .unwrap();
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
