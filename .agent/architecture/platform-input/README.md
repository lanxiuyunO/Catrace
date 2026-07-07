# 平台输入层

跨平台键鼠监听的模块组织与条件编译策略。

## 模块层级

```
lib.rs (run 入口)
├── 键盘监听线程: device_query::DeviceState::on_key_down()
├── 鼠标采样线程: device_query::DeviceState::get_mouse()
├── 音频检测线程: media_audio::is_media_active() (Windows WASAPI)
└── 每分钟结算: tokio::time::interval(60s)
```

## 数据流

```
键盘线程 → state.count++ (Mutex)
鼠标线程 → state.count++ (Mutex)
音频线程 → state.audio_state (Arc<AtomicBool>)
          ↓
每分钟结算 tick → 读取 state.count + audio_state
          → 判定活跃/休息 → 写入 SQLite
          → 检查提醒条件 → 弹通知
```

## 关键约定

- 键盘/鼠标计数存在 `ActivityState` 的 `Mutex` 内，每分钟结算时读取并归零
- 键盘用回调事件驱动（`on_key_down`），非轮询
- 鼠标用工位轮询（2 秒间隔），因为 `device_query` 不提供鼠标移动事件
- 所有键鼠监听线程在 `run()` 函数中 `thread::spawn`，生命周期跟随进程
- macOS 上 `device_query` 键盘监听**必须在主线程**；当前实现 spawn 到独立线程，实测可行但依赖 `device_query` 内部行为

## 新增系统输入源时的修改点

1. `lib.rs` — 新增线程或回调，活动计数累加到 `ActivityState`
2. `media_audio.rs` — 若为音频类检测，遵循现有模块模式
3. `AGENTS.md` — 更新测试策略、技术栈
