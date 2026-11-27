# Island 项目说明

Island 是 Windows x64 桌面消息提醒工具。展开首页显示最近三条提醒的应用名称，记录保存在本次运行的内存中。

微信、企业微信、飞书和钉钉使用任务栏提醒。Windows 通知需要在设置中授权。设置存放在 %APPDATA%\Island。

构建命令为 `cargo build --locked --release --bin message-island`，可执行文件为 `target\release\message-island.exe`。
