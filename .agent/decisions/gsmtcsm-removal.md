# 放弃 GSMTCSM 媒体检测

## 背景

Windows 上最初使用 `GlobalSystemMediaTransportControlsSessionManager`（GSMTCSM）检测媒体播放状态，结合 WASAPI 音频检测双重判定。

## 决策：彻底移除 GSMTCSM

**理由：**

- GSMTCSM 只能检测暴露媒体会话的应用，很多直播客户端、网页播放器不暴露
- GSMTCSM 的 Playing/Paused 状态与"用户是否在看屏幕"不完全对应
- 引入 WASAPI 音频检测后，"有没有声音"成为更稳定的入口条件
- 结合音频输出进程排除列表，逻辑更简单、可预测

## 最终方案

Windows 上仅用 WASAPI 音频检测：
1. 枚举系统音频输出会话
2. 无音频输出 → 不活跃（接受静音看视频被误判）
3. 有音频输出 → 检查进程名是否在排除列表中 → 非排除即为活跃

## 时间

2026 年 6-7 月。
