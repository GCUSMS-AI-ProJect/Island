# 消息灵动岛复刻版

这是基于小糖岛仔 MIT 源码 `15e7b7da` 制作的独立 Windows x64 项目。
沿用岛仔的 Rust/egui 界面、Windows 字体加载、刘海轮廓、动效、贴边窗口及未读消息侧栏。
展开首页显示最近三条微信等任务栏提醒或 Windows 通知的来源。来源记录只在运行期间保存在内存里，首页只显示应用名。
微信、企业微信、飞书和钉钉的任务栏提醒默认开启。Windows 通知读取仍需在通知设置里自行授权。
设置存放在 `%APPDATA%\MessageIslandReplica`。程序使用独立实例和自启动条目。

## 构建

在安装 Rust MSVC 工具链、Visual Studio C++ Build Tools 和 Windows SDK 后，在项目目录运行：

```powershell
cargo build --locked --release --bin message-island
```

产物位于 `target\release\message-island.exe`。
上游源码和资源许可见 `LICENSE`。
