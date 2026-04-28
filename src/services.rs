//! 异步业务服务层：目前仅包含模拟登录。

use std::time::Duration;

/// 模拟异步登录请求
pub async fn fake_login(username: String, password: String) -> Result<String, String> {
    tokio::time::sleep(Duration::from_millis(1200)).await;
    if username == "admin" && password == "123456" {
        Ok(format!("登录成功，欢迎 {}", username))
    } else {
        Err("用户名或密码错误".into())
    }
}
