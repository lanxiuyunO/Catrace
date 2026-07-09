# 自动更新检查

## 涉及文件

- `src-tauri/src/lib.rs` — 启动 3 秒后通过 `tauri-plugin-updater` 检查更新
- `src-tauri/src/reminder_toast.rs` — 创建更新 Toast
- `src/views/ReminderToast.vue` — 更新卡片 UI

## 行为

- 应用启动 3 秒后异步检查一次（携带 `X-AccessKey` 头）
- 有新版本 → 右下角橙色更新 Toast
- 卡片不自动关，含「查看详情」（展开 changelog）和「立即更新」（下载+安装+重启）
- 下载中有进度条
- 检查失败仅日志，不阻断启动
- 整个生命周期只检查一次
