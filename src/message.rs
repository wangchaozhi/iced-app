//! 应用消息与状态定义。

#[derive(Debug, Clone)]
pub enum Message {
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

/// 登录流程的当前状态
#[derive(Default)]
pub enum Status {
    #[default]
    Idle,
    Loading,
    Success(String),
    Failed(String),
}
