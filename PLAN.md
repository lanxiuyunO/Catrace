# Catrace 开发计划

## 项目概述

后台静默运行的桌面应用，监听用户键鼠活动，判断工作/休息状态，连续工作时间过长时提醒休息。

## 核心逻辑

1. **采样**：每 2 秒检查一次光标位置；全局监听键盘按下事件（触发后 2s 内去重）
2. **分钟判定**：60 秒内活动次数 ≥ 3，则该分钟标记为 **活跃**，否则为 **休息**
3. **窗口检测**：每分钟获取当前焦点窗口进程名，按用户分类（工作/娱乐/未知）
4. **滑动窗口**：每分钟检查前面 `window_minutes` 分钟窗口
   - 窗口内存在连续 `break_minutes` 分钟休息 → 用户在休息，不提醒
   - 否则 → 算连续活跃，弹出系统通知提醒休息

## 配置项

| 配置名 | 说明 | 默认 |
|--------|------|------|
| `window_minutes` | 工作窗口长度（分钟） | 45 |
| `break_minutes` | 连续休息多少分钟算断开（分钟） | 5 |
| `app_categories` | 应用分类名单 JSON | `{}` |

## 技术栈

| 层 | 技术 |
|----|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| 后端(Rust) | rdev + device_query + rusqlite + tokio |
| 图表 | ECharts |

## 数据库设计

```sql
-- 每分钟记录
CREATE TABLE records (
    timestamp INTEGER PRIMARY KEY,  -- 整分钟时间戳
    is_active INTEGER,              -- 0 = 休息, 1 = 活跃
    process_name TEXT,              -- 当前焦点窗口进程名
    category TEXT                   -- work / entertainment / unknown
);

-- 配置键值对
CREATE TABLE settings (
    key TEXT PRIMARY KEY,
    value TEXT
);
```

## Rust 模块结构

```
src/
├── main.rs              -- Tauri 入口，setup
├── input/
│   ├── mod.rs           -- 启动/停止监听
│   ├── keyboard.rs      -- rdev 全局键盘 hook（2s debounce）
│   └── mouse.rs         -- tokio interval 2s 查光标位置
├── engine/
│   ├── mod.rs           -- 每分钟活跃判定 + 滑动窗口检测
│   └── window.rs        -- active-win-pos-rs 查焦点窗口
├── db.rs                -- rusqlite 读写封装
├── notify.rs            -- tauri::notification 封装
└── commands.rs          -- #[tauri::command] 暴露给前端
```

## 前端结构

```
src/
├── views/
│   ├── Dashboard.vue    -- 今日时间轴 + 统计看板
│   └── Settings.vue     -- 两个滑块 + 应用分类名单编辑
├── components/
│   └── Timeline.vue     -- 24h 色块时间轴
├── api/
│   └── tauri.ts         -- invoke 调用 Rust 命令的封装
└── App.vue              -- 路由/布局
```

## 开发计划（8 步）

| 步骤 | 内容 | 验证方式 |
|------|------|----------|
| 1 | Rust 裸跑：rdev 键盘监听 + 2s 光标采样，console 打印事件 | `cargo run` 看输出稳定 |
| 2 | 加每分钟活跃判定，写入 SQLite | 查 DB 有每分钟记录 |
| 3 | 滑动窗口算法：读 `window_minutes` 窗口，检测连续 `break_minutes` 休息，触发系统通知 | 手动构造时间数据测试通知弹出 |
| 4 | Tauri 套壳，前端 Vue 3 + 路由搭建 | `pnpm tauri dev` 窗口正常 |
| 5 | 前端 Settings 页：滑块改 `window_minutes` / `break_minutes`，invoke 传 Rust 生效 | 改配置后算法响应 |
| 6 | 前端 Dashboard：读 DB 画 24h 时间轴 + 今日统计 | 看到彩色时间块 |
| 7 | 系统托盘图标 + 开机自启 | 托盘右键菜单正常 |
| 8 | 应用分类名单：进程名 → work/entertainment，影响分类统计 | 切窗口看 category 变 |

## 边界情况

- 浏览器无法区分标签页，整个浏览器进程归用户设定的一类
- 监听线程崩溃应自动重启，前端显示状态
- 通知只在用户处于"连续活跃"状态时触发，休息期间不打扰
