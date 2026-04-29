//! 跨平台适配层：按目标操作系统选择 wgpu 后端与窗口装饰/透明策略。
//!
//! - Windows：优先 Vulkan，回退 GL，最后 DX12。规避 DX12 下透明窗口黑屏。
//! - macOS：使用 Metal；启用原生装饰（红绿灯）以获得最佳体验，关闭自绘透明。
//! - Linux：X11 下启用透明；Wayland 下关闭透明避免部分合成器黑背景。

/// 窗口视觉策略
pub struct WindowStyle {
    pub transparent: bool,
    pub decorations: bool,
}

/// 根据平台初始化 wgpu 后端环境变量，并返回推荐的窗口视觉策略。
pub fn init() -> WindowStyle {
    set_backend_env();
    window_style()
}

#[cfg(target_os = "windows")]
fn set_backend_env() {
    // Windows 下 DX12 的 CompositeAlphaMode 多为 Auto，透明窗口易出现黑边/黑底，
    // 优先使用 Vulkan / GL，DX12 作为兜底。
    if std::env::var_os("WGPU_BACKEND").is_none() {
        std::env::set_var("WGPU_BACKEND", "vulkan,gl,dx12");
    }
}

#[cfg(target_os = "macos")]
fn set_backend_env() {
    // macOS 只应使用 Metal 后端。
    if std::env::var_os("WGPU_BACKEND").is_none() {
        std::env::set_var("WGPU_BACKEND", "metal");
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn set_backend_env() {
    // Linux / BSD：优先 Vulkan，回退 GL。
    if std::env::var_os("WGPU_BACKEND").is_none() {
        std::env::set_var("WGPU_BACKEND", "vulkan,gl");
    }
}

#[cfg(target_os = "windows")]
fn window_style() -> WindowStyle {
    WindowStyle {
        transparent: true,
        decorations: false,
    }
}

#[cfg(target_os = "macos")]
fn window_style() -> WindowStyle {
    // macOS 使用原生装饰与红绿灯按钮，体验更一致；关闭透明避免阴影/圆角异常。
    WindowStyle {
        transparent: false,
        decorations: true,
    }
}

#[cfg(all(unix, not(target_os = "macos")))]
fn window_style() -> WindowStyle {
    // Wayland 会话下部分合成器对无边框透明窗口支持不一致，保守关闭透明。
    let is_wayland = std::env::var("XDG_SESSION_TYPE")
        .map(|v| v.eq_ignore_ascii_case("wayland"))
        .unwrap_or(false);

    WindowStyle {
        transparent: !is_wayland,
        decorations: false,
    }
}

/// 当前平台是否建议使用 `iced::window::drag` 进行自绘拖动。
/// macOS 启用原生装饰后由标题栏处理拖动，无需触发。
pub fn supports_custom_drag() -> bool {
    !cfg!(target_os = "macos")
}
