# bad_player

一个终端视频播放器

## 使用

使用前请确认你的系统在 Windows 10 及以上

由于 ffmpeg 在本程序中出现问题，请先将视频导出为图片序列：

```bash
ffmpeg -i video.mp4 /path/to/images/video-%06d.png
```

然后使用如下命令执行：

```bash
cargo run -- -i /path/to/images
```

更多选项请参阅 `help`

## TODOs

- [ ] 内存占用优化
- [x] 播放优化