# ShenEternity-MusicPlayer
## 这是一个跨平台音乐播放器
分server(服务端)和client(客户端);

### Server
数据库: 
数据库非常简单，就一个toml文件
```toml
[songs.0]
id = 0 
title = "说了再见"
artist = "周杰伦"
album = "跨时代"
path = "/home/eternity/Music/音频文件/说了再见.mp3"
image = "/home/eternity/Music/专辑图片/周杰伦-跨时代.jpg"
duration = 282.83

[songs.1]
id = 1
title = "走狗"
artist = "周柏豪"
album = "Beginning"
path = "/home/eternity/Music/音频文件/走狗.mp3"
image = "/home/eternity/Music/专辑图片/周柏豪-Beginning.webp"
duration = 249.35
```
一眼就能看懂每个字段的意义，不多赘述。

监听ip和端口:
默认使用 127.0.0.1:3000
更改 main.rs 内的 bind 即可

api接口:
- // http .........   /api/songs // /api/songs?keyword=xxx     获取歌单，搜索
- // http .........   /api/songs/{id}/stream                   获取音频流
- // http .........   /api/songs/{id}/lyrics                   获取歌词/目前客户端只支持lrc格式的解析
- // http .........   /api/songs/{id}/image                    获取图片
- // http .........   /api/songs/{id}/data                     获取歌曲数据


### Client
这个没什么可以讲的

