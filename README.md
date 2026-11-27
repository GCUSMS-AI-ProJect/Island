# AirLand · 爱尔栏

适用于 Windows x64 的桌面消息提醒工具。平时以刘海或胶囊形态停靠在屏幕边缘，显示时钟和未读提示。鼠标移近后展开，查看最近的消息来源。

## 消息从哪里来

- **微信、企业微信、飞书、钉钉**通过任务栏闪动触发提醒。客户端需要开启任务栏闪动，岛上的未读图标可用于切回对应窗口。
- **QQ 和其他应用**可以通过 Windows 系统通知接入。在应用中开启系统通知，再到爱尔栏的「消息提醒」设置里连接并授权。

展开后的消息首页显示最近三条提醒的应用名称。消息来源记录只保存在本次运行的内存中，通知正文预览默认关闭。设置保存在 `%APPDATA%\Island`。

## 从源码构建

准备 Rust MSVC 工具链、Visual Studio C++ Build Tools 和 Windows SDK，然后在仓库根目录执行：

```powershell
cargo build --locked --release --bin message-island
```

启动生成的 `target\release\message-island.exe` 即可。

## 许可

项目采用 MIT 许可证。[MIT 许可证](LICENSE) · [第三方许可说明](THIRD_PARTY_NOTICES.txt)
