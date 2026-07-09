# 键鼠输入监听

后台静默监听键鼠活动，实时累积活动次数，供每分钟结算判定活跃/休息。

## 涉及文件

- `src-tauri/src/lib.rs` — 键盘监听线程 + 鼠标采样线程 + 每分钟结算
- `src-tauri/src/reminder_toast.rs` — 仅用 `DeviceQuery::get_mouse()` 获取屏幕尺寸计算 Toast 窗口位置

## 键盘监听

所有平台统一使用 `device_query::DeviceState::on_key_down()` 事件回调。
2 秒去重：同一秒内多次按键只计 1 次活动。

## 鼠标采样

每 2 秒轮询 `DeviceState::get_mouse().coords`，位置变动则活动 + 1。

## 子文档

- [device_query 选择](device_query-choice.md) — 为什么不用 rdev
