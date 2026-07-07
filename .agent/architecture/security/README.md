# 安全与隐私

- 全局键鼠监听仅计数，不记录按键内容或鼠标轨迹坐标
- 数据库文件保存在 `app_data_dir/catrace.db`，不上传
- 应用启动时向 UpgradeLink 上报 `app_start`（版本号、平台、架构、匿名设备标识），用于统计分析；不上传活动记录
- `device_query` / `active-win-pos-rs` 需要系统权限（macOS Accessibility / Windows UI Access）
