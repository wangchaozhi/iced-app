# Iced App

基于 [Iced](https://github.com/iced-rs/iced) 框架的跨平台 Rust GUI 桌面应用。

## 功能特性

- 🎨 深色/浅色主题切换 (TokyoNight / Light)
- 👁 密码显示/隐藏切换
- 🌐 中文字体支持（思源黑体 Noto Sans SC）
- 😀 表情符号支持（Noto Color Emoji）
- ⏳ 异步登录模拟（Tokio 运行时）
- 📦 字体嵌入二进制，开箱即用
- 🪟 发布模式隐藏控制台窗口

## 技术栈

| 组件 | 版本 |
|------|------|
| Rust | 1.91.0 |
| Iced | 0.14 |
| Tokio | 1.x |

## 运行项目

### 前置条件

- [Rust](https://rustup.rs/) (建议使用 `rustup` 安装)

### 开发模式

```bash
cargo run
```

### 发布构建

```bash
cargo build --release
```

发布构建启用了 LTO、最高优化级别和符号剥离，生成的二进制体积更小、性能更好。

## 项目结构

```
├── assets/
│   ├── NotoColorEmoji-Regular.ttf   # Emoji 字体
│   └── NotoSansSC-Regular.otf       # 思源黑体
├── src/
│   └── main.rs                      # 应用主入口
├── Cargo.toml
└── Cargo.lock
```

## 测试登录

默认测试账号：

- 用户名：`admin`
- 密码：`123456`

## License

MIT
