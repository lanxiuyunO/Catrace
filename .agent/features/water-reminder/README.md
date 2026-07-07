# 喝水提醒

独立的喝水提醒系统，与休息提醒解耦。

## 涉及文件

- `src-tauri/src/water.rs` — 状态机 `WaterReminderState` + 命令 + Toast 通知 + 每分钟结算检查
- `src-tauri/src/lib.rs` — 每分钟活跃结算时调用 `water::check_and_notify()`
- `src/components/WaterWidget.vue` — Dashboard 喝水统计小组件
- `src/components/settings/WaterSettingsCard.vue` — 设置页喝水提醒卡片

## 触发逻辑

- 仅当前分钟**活跃**时检查；休息期间不提醒
- 距上次喝水超过 `water_interval_minutes` → Toast 提醒
- 触发后自动按 `water_interval_minutes` 设置 snooze
- 用户可点 Toast「已喝水」或 WaterWidget「+1 次喝水」记录

## UI

- WaterWidget 仅在 `water_reminder_enabled` 为 true 时显示
- 显示今日次数、最近一次喝水时间（每秒刷新）、时间轴
- 喝水 Toast 采用与 WaterWidget 统一的**蓝色主题**

## 配置

| 配置名 | 说明 | 默认值 |
|--------|------|--------|
| `water_reminder_enabled` | 开启喝水提醒 | true |
| `water_interval_minutes` | 多久未喝水提醒（分钟） | 60 |

## 测试

`water.rs` 3 个测试（snooze / 去重 / 喝水后清除 snooze）+ `db.rs` 1 个测试（喝水记录 CRUD）。
