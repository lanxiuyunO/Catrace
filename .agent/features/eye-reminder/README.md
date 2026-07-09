# 护眼提醒

参照喝水提醒架构实现的独立护眼提醒系统，按间隔提醒用户看向远处休息眼睛。

## 涉及文件

- `src-tauri/src/eye.rs` — 状态机 `EyeReminderState`、设置读写、命令、每分钟结算检查
- `src-tauri/src/lib.rs` — 每分钟活跃结算时调用 `eye::check_and_notify()`
- `src/components/settings/EyeSettingsCard.vue` — 设置页护眼提醒卡片
- `src/views/ReminderToast.vue` — `kind="eye"` Toast 卡片与绿色主题
- `src/api/tauri.ts` — 前端 API 封装
- `src/i18n/locales/zh-CN.ts` / `en-US.ts` — 中英文字符串

## 触发逻辑

- 仅当前分钟**活跃**时检查；休息期间不提醒
- 距上次护眼提醒超过 `eye_interval_minutes` → Toast 提醒
- 触发后自动按 `eye_interval_minutes` 设置 snooze
- 用户可点 Toast「稍后 5 分钟」「稍后 10 分钟」「跳过本次」

## UI

- 设置卡片使用绿色主题，与喝水提醒蓝色主题区分
- Toast 卡片左侧脉冲点、进度条、操作按钮均为绿色系
- 中文界面显示「护眼提醒」，英文为 "Eye Care Reminder"

## 配置

| 配置名 | 说明 | 默认值 |
|--------|------|--------|
| `eye_reminder_enabled` | 开启护眼提醒 | true |
| `eye_interval_minutes` | 连续活跃多久提醒护眼（分钟） | 20 |

## 测试

`eye.rs` 包含 2 个测试：snooze 状态判断、`can_send_reminder` 1 秒去重。

## 相关

- [[water-reminder]] — 同类提醒系统的实现参考
- [[toast-window]] — Toast 通知承载窗口
