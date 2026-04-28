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

项目采用模块化组织，职责清晰分层：

```
├── assets/
│   ├── NotoColorEmoji-Regular.ttf   # Emoji 字体
│   └── NotoSansSC-Regular.otf       # 思源黑体
├── src/
│   ├── main.rs                      # 入口：iced 应用初始化、字体装配
│   ├── app.rs                       # LoginApp：状态、update 逻辑、view 聚合
│   ├── message.rs                   # Message / Status 类型定义
│   ├── services.rs                  # 异步业务层（模拟登录等）
│   ├── fonts.rs                     # 嵌入字体常量与默认字体名
│   └── views/
│       ├── mod.rs                   # 视图模块入口
│       └── login.rs                 # 登录界面组件（顶栏/输入/按钮/底栏）
├── Cargo.toml
└── Cargo.lock
```

### 模块职责

| 模块 | 说明 |
|------|------|
| `main` | 仅负责应用启动、窗口/主题/字体等全局配置 |
| `app` | 应用状态 `LoginApp` 与 `update` 消息处理 |
| `message` | `Message` 枚举与 `Status` 状态定义 |
| `services` | 异步业务接口（当前为 `fake_login`，后续接入真实后端） |
| `views::login` | 登录卡片 UI 构建，拆分为多个小函数便于维护 |
| `fonts` | 编译期嵌入的字体字节与默认字体族名 |

新增页面时：在 `views/` 下新建模块并在 `app::view` 中分发即可。

## 测试登录

默认测试账号：

- 用户名：`admin`
- 密码：`123456`

## License

MIT
