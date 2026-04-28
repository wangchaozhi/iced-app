#![windows_subsystem = "windows"] // 发布时隐藏控制台（调试时可注释）

use iced::widget::{button, column, container, mouse_area, row, stack, text, text_input, Space};
use iced::{
    alignment::{Horizontal, Vertical},
    Background, Border, Color, Element, Length, Task, Theme,
};

// 嵌入中文字体（思源黑体 Noto Sans SC），编译时打包进二进制
const NOTO_SANS_SC: &[u8] = include_bytes!("../assets/NotoSansSC-Regular.otf");
// 嵌入 Emoji 字体（Noto Color Emoji），作为表情符号回退字体
const NOTO_COLOR_EMOJI: &[u8] = include_bytes!("../assets/NotoColorEmoji-Regular.ttf");

fn main() -> iced::Result {
    // 优先使用 Vulkan/GL 后端，因为 Windows 下 DX12 wgpu 通常只提供 CompositeAlphaMode::Auto，
    // 无法真正透明，导致圆角外的区域变黑。
    if std::env::var_os("WGPU_BACKEND").is_none() {
        std::env::set_var("WGPU_BACKEND", "vulkan,gl,dx12");
    }

    iced::application(LoginApp::default, LoginApp::update, LoginApp::view)
        .title("登录 - Rust GUI")
        .theme(LoginApp::theme)
        .style(|_state, _theme| iced::theme::Style {
            background_color: Color::TRANSPARENT,
            text_color: Color::WHITE,
        })
        .window_size((380.0, 560.0))
        .centered()
        .decorations(false)
        .transparent(true)
        .font(NOTO_SANS_SC)
        .font(NOTO_COLOR_EMOJI)
        .default_font(iced::Font::with_name("Noto Sans SC"))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    UsernameChanged(String),
    PasswordChanged(String),
    TogglePasswordVisible,
    ToggleRememberMe(bool),
    LoginPressed,
    LoginFinished(Result<String, String>),
    ToggleTheme,
    ClearError,
    CloseWindow,
    StartDrag,
}

#[derive(Default)]
enum Status {
    #[default]
    Idle,
    Loading,
    Success(String),
    Failed(String),
}

struct LoginApp {
    username: String,
    password: String,
    password_visible: bool,
    remember_me: bool,
    status: Status,
    dark_mode: bool,
}

impl Default for LoginApp {
    fn default() -> Self {
        Self {
            username: String::new(),
            password: String::new(),
            password_visible: false,
            remember_me: true,
            status: Status::Idle,
            dark_mode: true,
        }
    }
}

impl LoginApp {
    fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::TokyoNight
        } else {
            Theme::Light
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::UsernameChanged(v) => {
                self.username = v;
                self.status = Status::Idle;
                Task::none()
            }
            Message::PasswordChanged(v) => {
                self.password = v;
                self.status = Status::Idle;
                Task::none()
            }
            Message::TogglePasswordVisible => {
                self.password_visible = !self.password_visible;
                Task::none()
            }
            Message::ToggleRememberMe(v) => {
                self.remember_me = v;
                Task::none()
            }
            Message::ToggleTheme => {
                self.dark_mode = !self.dark_mode;
                Task::none()
            }
            Message::ClearError => {
                self.status = Status::Idle;
                Task::none()
            }
            Message::CloseWindow => iced::exit(),
            Message::StartDrag => {
                iced::window::latest().and_then(iced::window::drag)
            }
            Message::LoginPressed => {
                if self.username.trim().is_empty() || self.password.is_empty() {
                    self.status = Status::Failed("用户名和密码不能为空".into());
                    return Task::none();
                }
                self.status = Status::Loading;
                let user = self.username.clone();
                let pwd = self.password.clone();
                Task::perform(fake_login(user, pwd), Message::LoginFinished)
            }
            Message::LoginFinished(result) => {
                self.status = match result {
                    Ok(msg) => Status::Success(msg),
                    Err(e) => Status::Failed(e),
                };
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let close_btn = button(text("✕").size(16))
            .on_press(Message::CloseWindow)
            .padding([2, 8])
            .style(button::text);

        let top_bar = row![Space::new().width(Length::Fill), close_btn]
            .align_y(Vertical::Center);

        let title = text("欢迎回来")
            .size(32)
            .align_x(Horizontal::Center)
            .width(Length::Fill);

        let subtitle = text("请登录您的账户以继续")
            .size(14)
            .align_x(Horizontal::Center)
            .width(Length::Fill)
            .style(|theme: &Theme| text::Style {
                color: Some(theme.extended_palette().background.weak.text),
            });

        let username_input = text_input("用户名 / 邮箱", &self.username)
            .on_input(Message::UsernameChanged)
            .on_submit(Message::LoginPressed)
            .padding(12)
            .size(15);

        let password_input = {
            let mut input = text_input("密码", &self.password)
                .on_input(Message::PasswordChanged)
                .on_submit(Message::LoginPressed)
                .padding(iced::Padding {
                    top: 12.0,
                    right: 44.0,
                    bottom: 12.0,
                    left: 12.0,
                })
                .size(15);
            if !self.password_visible {
                input = input.secure(true);
            }
            input
        };

        let eye_btn = button(text(if self.password_visible { "🙈" } else { "👁" }).size(16))
            .on_press(Message::TogglePasswordVisible)
            .padding([4, 8])
            .style(button::text);

        let eye_overlay = container(eye_btn)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Right)
            .align_y(Vertical::Center)
            .padding([0, 4]);

        let password_row = stack![password_input, eye_overlay];

        let remember = row![
            iced::widget::checkbox(self.remember_me)
                .label("记住我")
                .on_toggle(Message::ToggleRememberMe),
            Space::new().width(Length::Fill),
            button(text("忘记密码?").size(13))
                .on_press(Message::ClearError)
                .style(button::text)
                .padding(0),
        ]
        .align_y(Vertical::Center);

        let login_button: Element<Message> = match &self.status {
            Status::Loading => button(
                text("登录中...")
                    .size(16)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .padding(14)
            .width(Length::Fill)
            .style(button::secondary)
            .into(),
            _ => button(
                text("登 录")
                    .size(16)
                    .align_x(Horizontal::Center)
                    .width(Length::Fill),
            )
            .on_press(Message::LoginPressed)
            .padding(14)
            .width(Length::Fill)
            .style(button::primary)
            .into(),
        };

        let status_view: Element<Message> = match &self.status {
            Status::Idle | Status::Loading => Space::new().height(20).into(),
            Status::Success(msg) => text(format!("✓ {}", msg))
                .size(14)
                .style(|_: &Theme| text::Style {
                    color: Some(Color::from_rgb(0.2, 0.8, 0.4)),
                })
                .into(),
            Status::Failed(msg) => text(format!("✗ {}", msg))
                .size(14)
                .style(|_: &Theme| text::Style {
                    color: Some(Color::from_rgb(0.95, 0.35, 0.4)),
                })
                .into(),
        };

        let theme_toggle = button(text(if self.dark_mode { "☀ 浅色" } else { "🌙 深色" }).size(12))
            .on_press(Message::ToggleTheme)
            .style(button::text)
            .padding(4);

        let footer = row![
            text("还没有账户? ").size(13),
            button(text("立即注册").size(13))
                .on_press(Message::ClearError)
                .style(button::text)
                .padding(0),
            Space::new().width(Length::Fill),
            theme_toggle,
        ]
        .align_y(Vertical::Center);

        let card_content = column![
            top_bar,
            title,
            subtitle,
            Space::new().height(24),
            username_input,
            password_row,
            remember,
            Space::new().height(8),
            login_button,
            status_view,
            Space::new().height(8),
            footer,
        ]
        .spacing(14)
        .padding(32)
        .width(Length::Fill);

        let card = container(card_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.weak.color)),
                    border: Border {
                        color: palette.background.strong.color,
                        width: 1.0,
                        radius: 16.0.into(),
                    },
                    ..Default::default()
                }
            });

        let page = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2);

        mouse_area(page).on_press(Message::StartDrag).into()
    }
}

// 模拟异步登录请求
async fn fake_login(username: String, password: String) -> Result<String, String> {
    tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
    if username == "admin" && password == "123456" {
        Ok(format!("登录成功，欢迎 {}", username))
    } else {
        Err("用户名或密码错误".into())
    }
}
