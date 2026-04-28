#![windows_subsystem = "windows"] // 发布时隐藏控制台（调试时可注释）

mod app;
mod fonts;
mod message;
mod services;
mod views;

use iced::Color;

use crate::app::LoginApp;

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
        .font(fonts::NOTO_SANS_SC)
        .font(fonts::NOTO_COLOR_EMOJI)
        .default_font(iced::Font::with_name(fonts::DEFAULT_FONT_NAME))
        .run()
}
