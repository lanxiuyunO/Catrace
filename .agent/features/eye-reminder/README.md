# 护眼提醒

按间隔提醒用户看向远处休息眼睛，提醒卡片以 25 秒倒计时自动关闭，无需用户手动处理。

## 涉及文件

- `src-tauri/src/eye.rs` — 状态机 `EyeReminderState`、设置读写、命令、每分钟结算检查
- `src-tauri/src/lib.rs` — 每分钟活跃结算时调用 `eye::check_and_notify()`
- `src/components/EyeToastCard.vue` — 护眼提醒专用 Toast 卡片（标题 + 倒计时 + 进度条）
- `src/components/settings/EyeSettingsCard.vue` — 设置页护眼提醒卡片
- `src/views/ReminderToast.vue` — 通用 Toast 堆叠容器，调度 `EyeToastCard`
- `src/api/tauri.ts` — 前端 API 封装
- `src/i18n/locales/zh-CN.ts` / `en-US.ts` — 中英文字符串

## 触发逻辑

- 仅当前分钟**活跃**时检查；休息期间不提醒
- 距上次护眼提醒超过 `eye_interval_minutes` → 右下角 Toast
- 触发后记录 `eye_last_reminder_ts`，等待下一个间隔再次触发
- 同一秒内重复触发会被 `EyeReminderState::can_send_reminder` 过滤

## UI

- 设置卡片使用绿色主题，与喝水提醒蓝色主题区分
- Toast 卡片仅显示：脉冲点 + 标题、25 秒倒计时、绿色进度条、关闭按钮
- 倒计时位于进度条右侧，每秒刷新
- Hover 不暂停倒计时，到时间自动关闭
- 中文界面显示「护眼提醒」，英文为 "Eye Care Reminder"

## 配置

| 配置名 | 说明 | 默认值 |
|--------|------|--------|
| `eye_reminder_enabled` | 开启护眼提醒 | true |
| `eye_interval_minutes` | 连续活跃多久提醒护眼（分钟） | 20 |

## 测试

`eye.rs` 包含 1 个测试：`can_send_reminder` 1 秒去重。

## 相关

- [[water-reminder]] — 同类提醒系统的实现参考
- [[toast-window]] — Toast 通知承载窗口
