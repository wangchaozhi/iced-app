//! 嵌入式字体资源：编译时打包进二进制，避免中文/表情乱码。

/// 思源黑体（简体中文主字体）
pub const NOTO_SANS_SC: &[u8] = include_bytes!("../assets/NotoSansSC-Regular.otf");

/// Noto Color Emoji（Emoji 回退字体）
pub const NOTO_COLOR_EMOJI: &[u8] = include_bytes!("../assets/NotoColorEmoji-Regular.ttf");

/// 默认字体族名称
pub const DEFAULT_FONT_NAME: &str = "Noto Sans SC";
