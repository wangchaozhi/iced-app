// 仅在 Windows 下隐藏控制台子系统；其他平台忽略该属性。
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod app;
mod fonts;
mod message;
mod platform;
mod services;
mod views;

use iced::Color;

use crate::app::LoginApp;

fn main() -> iced::Result {
    // 按平台初始化 wgpu 后端环境变量，并得到推荐的窗口装饰/透明策略。
    let style = platform::init();

    // 透明窗口的背景色需为 TRANSPARENT；非透明时用纯色背景兜底，避免 Wayland 黑背景。
    let bg_color = if style.transparent {
        Color::TRANSPARENT
    } else {
        Color::from_rgb(0.08, 0.09, 0.12)
    };

    iced::application(LoginApp::default, LoginApp::update, LoginApp::view)
        .title("登录 - Rust GUI")
        .theme(LoginApp::theme)
        .style(move |_state, _theme| iced::theme::Style {
            background_color: bg_color,
            text_color: Color::WHITE,
        })
        .window_size((380.0, 560.0))
        .centered()
        .decorations(style.decorations)
        .transparent(style.transparent)
        .font(fonts::NOTO_SANS_SC)
        .font(fonts::NOTO_COLOR_EMOJI)
        .default_font(iced::Font::with_name(fonts::DEFAULT_FONT_NAME))
        .run()
}
