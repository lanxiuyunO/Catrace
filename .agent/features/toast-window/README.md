# Toast 提醒窗口

独立透明 WebviewWindow + Vue 卡片实现的右下角通知堆叠。

## 涉及文件

- `src-tauri/src/reminder_toast.rs` — Toast 窗口创建、位置计算、尺寸调整
- `src-tauri/src/window_manager/` — 无焦点显示（Windows `WS_EX_NOACTIVATE`）
- `src/views/ReminderToast.vue` — 前端卡片堆叠 + 动画

## 窗口特性

- 透明无边框 WebviewWindow，复用而非销毁
- 定位到工作区右下角，支持多屏
- Windows 不抢夺焦点（`WS_EX_NOACTIVATE` + `SW_SHOWNOACTIVATE`）
- macOS / Linux 回退到普通显示

## 卡片行为

- 新卡片右侧滑入，关闭时 FLIP 动画让下方卡片上移
- 8 秒自动消失，hover 暂停，离开恢复
- 「5分钟后提醒」「10分钟后提醒」「跳过本次」按钮
- 同一类非持久提醒（护眼/喝水）只保留一个，避免快速测试时堆叠
- 内容超出时 `.toast-stack` 可滚动，并自动滚动到底部

## 卡片类型（按 `kind` 区分主题）

| kind | 颜色 | 行为 |
|------|------|------|
| 休息提醒 | 紫色 | 8s 自动消失 |
| 喝水提醒 | 蓝色 | 8s 自动消失 |
| 护眼提醒 | 绿色 | 8s 自动消失 |
| 休息计时 | 绿色 | 不自动关闭，液体球动画，满 break_minutes 后继续累计 |
| 更新通知 | 橙色 | 不自动关闭，展开更新日志 + 下载进度条 |

## 调试

Debug 页开启 `toast_debug_mode` → Toast 窗口背景变半透明黄色，方便排查布局/点击。
