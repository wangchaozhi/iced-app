//! 应用主体：状态、更新逻辑与视图入口。

use iced::{Element, Task, Theme};

use crate::message::{Message, Status};
use crate::services::fake_login;
use crate::views::login::{self, LoginViewData};

pub struct LoginApp {
    pub username: String,
    pub password: String,
    pub password_visible: bool,
    pub remember_me: bool,
    pub status: Status,
    pub dark_mode: bool,
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
    pub fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::TokyoNight
        } else {
            Theme::Light
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
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
            Message::StartDrag => iced::window::latest().and_then(iced::window::drag),
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

    pub fn view(&self) -> Element<'_, Message> {
        login::view(LoginViewData {
            username: &self.username,
            password: &self.password,
            password_visible: self.password_visible,
            remember_me: self.remember_me,
            status: &self.status,
            dark_mode: self.dark_mode,
        })
    }
}
