# Popup 提醒：待决策移除

## 当前状态

前端设置页已隐藏「弹窗提醒」选项（`NotificationSettingsCard.vue` 中注释掉 popup 选项）。仅保留 toast / fullscreen 两种模式。

## 待后续评估

若 toast / fullscreen 两种模式足够，可彻底删除 popup 相关代码：

- 前端：`src/views/ReminderPopup.vue`、router 中 `/reminder-popup` 路由
- Rust：`lib.rs` 中 `reminder_mode == "popup"` 分支、`window_manager` 中 popup 相关逻辑
- i18n：`zh-CN.ts` / `en-US.ts` 中 popup 翻译键

## 决策

**先保留代码，仅隐藏入口**。避免后续反悔需要回滚。
