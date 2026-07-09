use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::{db, reminder_toast, ReminderWindowStore};

/// 视力提醒状态机（进程级，重启后重置）
#[derive(Default)]
pub struct EyeReminderState {
    /// 推迟提醒直到该时刻
    pub snooze_until: Option<Instant>,
    /// 最后一次发送视力提醒的时刻，用于防止同一分钟内重复触发
    pub last_reminder_sent: Option<Instant>,
}

impl EyeReminderState {
    pub fn is_snoozed(&self) -> bool {
        self.snooze_until.map_or(false, |t| t > Instant::now())
    }

    /// 距离上次发送是否已超过 1 秒，避免同一秒内重复弹窗
    pub fn can_send_reminder(&self) -> bool {
        self.last_reminder_sent
            .map_or(true, |t| t.elapsed() >= Duration::from_secs(1))
    }
}

// ---------- i18n helpers ----------

fn eye_notify_title(locale: &str) -> &'static str {
    match locale {
        "zh-CN" => "视力提醒",
        _ => "Eye Care Reminder",
    }
}

fn eye_notify_body(locale: &str) -> &'static str {
    match locale {
        "zh-CN" => "该让眼睛休息一下了，看看远处吧。",
        _ => "Time to rest your eyes. Look into the distance.",
    }
}

// ---------- 通知 ----------

pub fn show_eye_notification(
    app_handle: &tauri::AppHandle,
    locale: &str,
    store: &ReminderWindowStore,
) {
    let title = eye_notify_title(locale).to_string();
    let body = eye_notify_body(locale).to_string();
    reminder_toast::create_toast_window(app_handle, 0, &title, &body, "eye", store);
}

// ---------- 命令 ----------

#[tauri::command]
pub fn get_eye_settings(db: tauri::State<db::Db>) -> serde_json::Value {
    let enabled = db.get_setting("eye_reminder_enabled", "true") == "true";
    let interval: i64 = db
        .get_setting("eye_interval_minutes", "20")
        .parse()
        .unwrap_or(20);
    serde_json::json!({ "enabled": enabled, "interval_minutes": interval })
}

#[tauri::command]
pub fn set_eye_settings(
    enabled: bool,
    interval_minutes: i64,
    db: tauri::State<db::Db>,
) -> Result<(), String> {
    db.set_setting("eye_reminder_enabled", &enabled.to_string())
        .map_err(|e| e.to_string())?;
    db.set_setting("eye_interval_minutes", &interval_minutes.to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn snooze_eye_reminder(
    minutes: u64,
    state: tauri::State<Arc<Mutex<EyeReminderState>>>,
) {
    let mut s = state.lock().unwrap();
    s.snooze_until = Some(Instant::now() + Duration::from_secs(minutes * 60));
}

#[tauri::command]
pub fn skip_eye_reminder(
    db: tauri::State<db::Db>,
    state: tauri::State<Arc<Mutex<EyeReminderState>>>,
) {
    let eye_interval: u64 = db
        .get_setting("eye_interval_minutes", "20")
        .parse()
        .unwrap_or(20);
    let mut s = state.lock().unwrap();
    s.snooze_until = Some(Instant::now() + Duration::from_secs(eye_interval * 60));
}

#[tauri::command]
pub fn test_eye_notification(
    app_handle: tauri::AppHandle,
    db: tauri::State<db::Db>,
    store: tauri::State<ReminderWindowStore>,
) {
    let locale = db.get_setting("locale", "zh-CN");
    show_eye_notification(&app_handle, &locale, &store);
}

// ---------- 结算时检查 ----------

/// 在每分钟结算时检查是否需要弹出视力提醒。
/// 仅在当前分钟活跃时检查；休息时不会打扰。
pub fn check_and_notify(
    active: bool,
    db: &db::Db,
    eye_state: &Arc<Mutex<EyeReminderState>>,
    app_handle: &tauri::AppHandle,
    locale: &str,
    store: &ReminderWindowStore,
) {
    let eye_enabled = db.get_setting("eye_reminder_enabled", "true") == "true";
    if !active || !eye_enabled {
        return;
    }

    let eye_interval: i64 = db
        .get_setting("eye_interval_minutes", "20")
        .parse()
        .unwrap_or(20);
    let now_ts = chrono::Local::now().timestamp();
    let overdue = match db
        .get_setting("eye_last_reminder_ts", "")
        .parse::<i64>()
    {
        Ok(last_ts) if last_ts > 0 => now_ts - last_ts >= eye_interval * 60,
        _ => true,
    };

    if overdue {
        let mut state = eye_state.lock().unwrap();
        if !state.is_snoozed() && state.can_send_reminder() {
            state.last_reminder_sent = Some(Instant::now());
            state.snooze_until = Some(
                Instant::now() + Duration::from_secs((eye_interval as u64) * 60),
            );
            drop(state);
            let _ = db.set_setting("eye_last_reminder_ts", &now_ts.to_string());
            show_eye_notification(app_handle, locale, store);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_eye_state_snooze() {
        let mut state = EyeReminderState::default();
        assert!(!state.is_snoozed());

        state.snooze_until = Some(Instant::now() + Duration::from_secs(60));
        assert!(state.is_snoozed());

        state.snooze_until = Some(Instant::now() - Duration::from_secs(1));
        assert!(!state.is_snoozed());
    }

    #[test]
    fn test_eye_state_can_send_reminder() {
        let mut state = EyeReminderState::default();
        assert!(state.can_send_reminder());

        state.last_reminder_sent = Some(Instant::now());
        assert!(!state.can_send_reminder());

        thread::sleep(Duration::from_secs(2));
        assert!(state.can_send_reminder());
    }
}
