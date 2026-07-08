/// 统一日志模块：将所有后端日志通过 Tauri event 推送到前端控制台。
/// 使用方式：
///   log!("key", "info message {}", arg);
///   log!("key", "warn message {}", arg);   // 标签含 "warn" → console.warn
///   log!("key", "error message {}", arg);  // 标签含 "error/failed" → console.error
///
/// 每条日志同时输出到 stderr 和前端 `catrace-log` 事件。
use std::sync::OnceLock;
use tauri::Emitter;

static APP_HANDLE: OnceLock<tauri::AppHandle> = OnceLock::new();

pub fn init(app_handle: tauri::AppHandle) {
    let _ = APP_HANDLE.set(app_handle);
}

pub fn emit_log(tag: &str, level: &str, msg: String) {
    let ts = chrono::Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    // stderr 同时打印，方便开发者终端查看
    eprintln!("[{}] [{}] {}", tag, level, msg);

    if let Some(handle) = APP_HANDLE.get() {
        let _ = handle.emit(
            "catrace-log",
            serde_json::json!({
                "timestamp": ts,
                "tag": tag,
                "level": level,
                "message": msg,
            }),
        );
    }
}

#[macro_export]
macro_rules! log_info {
    ($tag:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log::emit_log($tag, "info", msg);
    }};
}

#[macro_export]
macro_rules! log_warn {
    ($tag:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log::emit_log($tag, "warn", msg);
    }};
}

#[macro_export]
macro_rules! log_error {
    ($tag:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        $crate::log::emit_log($tag, "error", msg);
    }};
}

/// 万能宏：根据 tag 内容自动判断 level
/// - tag 包含 "error" 或 "failed" → error
/// - tag 包含 "warn" → warn
/// - 其他 → info
#[macro_export]
macro_rules! log {
    ($tag:expr, $($arg:tt)*) => {{
        let msg = format!($($arg)*);
        let tag_str: &str = $tag;
        let level = if tag_str.to_lowercase().contains("error")
            || tag_str.to_lowercase().contains("failed")
        {
            "error"
        } else if tag_str.to_lowercase().contains("warn") {
            "warn"
        } else {
            "info"
        };
        $crate::log::emit_log(tag_str, level, msg);
    }};
}
