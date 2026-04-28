//! 登录界面视图。
//!
//! 将整个登录卡片的 UI 构建从 `LoginApp` 中抽离，便于维护与复用。

use iced::widget::{button, column, container, mouse_area, row, stack, text, text_input, Space};
use iced::{
    alignment::{Horizontal, Vertical},
    Background, Border, Color, Element, Length, Theme,
};

use crate::message::{Message, Status};

/// 登录视图所需的输入数据（来自 `LoginApp` 状态）
pub struct LoginViewData<'a> {
    pub username: &'a str,
    pub password: &'a str,
    pub password_visible: bool,
    pub remember_me: bool,
    pub status: &'a Status,
    pub dark_mode: bool,
}

/// 构建登录界面
pub fn view(data: LoginViewData<'_>) -> Element<'_, Message> {
    let top_bar = top_bar();
    let title = title();
    let subtitle = subtitle();
    let username_input = username_input(data.username);
    let password_row = password_row(data.password, data.password_visible);
    let remember = remember_row(data.remember_me);
    let login_button = login_button(data.status);
    let status_view = status_view(data.status);
    let footer = footer(data.dark_mode);

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

fn top_bar<'a>() -> Element<'a, Message> {
    let close_btn = button(text("✕").size(16))
        .on_press(Message::CloseWindow)
        .padding([2, 8])
        .style(button::text);

    row![Space::new().width(Length::Fill), close_btn]
        .align_y(Vertical::Center)
        .into()
}

fn title<'a>() -> Element<'a, Message> {
    text("欢迎回来")
        .size(32)
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .into()
}

fn subtitle<'a>() -> Element<'a, Message> {
    text("请登录您的账户以继续")
        .size(14)
        .align_x(Horizontal::Center)
        .width(Length::Fill)
        .style(|theme: &Theme| text::Style {
            color: Some(theme.extended_palette().background.weak.text),
        })
        .into()
}

fn username_input(value: &str) -> Element<'_, Message> {
    text_input("用户名 / 邮箱", value)
        .on_input(Message::UsernameChanged)
        .on_submit(Message::LoginPressed)
        .padding(12)
        .size(15)
        .into()
}

fn password_row(value: &str, visible: bool) -> Element<'_, Message> {
    let mut input = text_input("密码", value)
        .on_input(Message::PasswordChanged)
        .on_submit(Message::LoginPressed)
        .padding(iced::Padding {
            top: 12.0,
            right: 44.0,
            bottom: 12.0,
            left: 12.0,
        })
        .size(15);
    if !visible {
        input = input.secure(true);
    }

    let eye_btn = button(text(if visible { "🙈" } else { "👁" }).size(16))
        .on_press(Message::TogglePasswordVisible)
        .padding([4, 8])
        .style(button::text);

    let eye_overlay = container(eye_btn)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Right)
        .align_y(Vertical::Center)
        .padding([0, 4]);

    stack![input, eye_overlay].into()
}

fn remember_row<'a>(remember_me: bool) -> Element<'a, Message> {
    row![
        iced::widget::checkbox(remember_me)
            .label("记住我")
            .on_toggle(Message::ToggleRememberMe),
        Space::new().width(Length::Fill),
        button(text("忘记密码?").size(13))
            .on_press(Message::ClearError)
            .style(button::text)
            .padding(0),
    ]
    .align_y(Vertical::Center)
    .into()
}

fn login_button<'a>(status: &Status) -> Element<'a, Message> {
    match status {
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
    }
}

fn status_view<'a>(status: &Status) -> Element<'a, Message> {
    match status {
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
    }
}

fn footer<'a>(dark_mode: bool) -> Element<'a, Message> {
    let theme_toggle = button(text(if dark_mode { "☀ 浅色" } else { "🌙 深色" }).size(12))
        .on_press(Message::ToggleTheme)
        .style(button::text)
        .padding(4);

    row![
        text("还没有账户? ").size(13),
        button(text("立即注册").size(13))
            .on_press(Message::ClearError)
            .style(button::text)
            .padding(0),
        Space::new().width(Length::Fill),
        theme_toggle,
    ]
    .align_y(Vertical::Center)
    .into()
}
