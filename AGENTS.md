# Catrace — Agent Guide

> 本文档面向 AI 编程助手。当前项目处于**规划阶段**，尚未开始编码。以下信息全部来源于仓库中已存在的 `README.md` 与 `PLAN.md`，不含推测或假设。

---

## 项目概述

Catrace 是一款计划中的桌面端工具，用于帮助用户平衡工作与休息。

- **核心功能**：后台静默监听键鼠活动，判断用户是否处于连续工作状态；当连续活跃时间超过阈值时，通过系统通知提醒用户休息。
- **隐私承诺**：不偷拍屏幕、不上传数据，所有信息保存在用户本地。
- **当前状态**：仅有产品说明文档（`README.md`）和开发计划（`PLAN.md`），**尚未创建任何源码文件、构建配置或依赖目录**。

---

## 仓库现状

当前仓库根目录下只有以下文件：

```
.
├── README.md   # 产品说明（中文）
└── PLAN.md     # 开发计划与技术架构（中文）
```

没有 `package.json`、`Cargo.toml`、`pyproject.toml`、`tsconfig.json`、`vite.config.ts`、`tauri.conf.json` 等任何构建或配置文件。

---

## 计划中的技术栈（来自 PLAN.md）

| 层级 | 选型 |
|------|------|
| 桌面框架 | Tauri 2 |
| 前端 | Vue 3 + TypeScript + Vite |
| 图表库 | ECharts |
| 后端（Rust）| rdev（全局键盘监听）、device_query（鼠标采样）、rusqlite（本地数据库）、tokio（异步运行时） |

---

## 计划中的核心逻辑

1. **采样**
   - 每 2 秒检查一次鼠标光标位置。
   - 全局监听键盘按下事件，2 秒内去重。
2. **分钟判定**
   - 60 秒内活动次数 ≥ 3 → 该分钟标记为**活跃**；否则标记为**休息**。
3. **窗口检测**
   - 每分钟获取当前焦点窗口的进程名，按用户配置分类为 work / entertainment / unknown。
4. **滑动窗口提醒**
   - 检查前面 `window_minutes`（默认 45）分钟的窗口。
   - 若窗口内存在连续 `break_minutes`（默认 5）分钟休息，则认为用户在休息，不提醒。
   - 否则视为连续活跃，弹出系统通知。

---

## 计划中的配置项

| 配置名 | 说明 | 默认值 |
|--------|------|--------|
| `window_minutes` | 工作窗口长度（分钟） | 45 |
| `break_minutes` | 连续休息多少分钟算断开（分钟） | 5 |
| `app_categories` | 应用分类名单（JSON） | `{}` |

---

## 计划中的目录结构

### Rust 后端（Tauri 侧）

```
src/
├── main.rs              -- Tauri 入口，setup
├── input/
│   ├── mod.rs           -- 启动/停止监听
│   ├── keyboard.rs      -- rdev 全局键盘 hook（2s debounce）
│   └── mouse.rs         -- tokio interval 2s 查光标位置
├── engine/
│   ├── mod.rs           -- 每分钟活跃判定 + 滑动窗口检测
│   └── window.rs        -- 查焦点窗口进程名
├── db.rs                -- rusqlite 读写封装
├── notify.rs            -- tauri::notification 封装
└── commands.rs          -- #[tauri::command] 暴露给前端
```

### 前端（Vue 3）

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

### 数据库（SQLite）

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

---

## 计划中的开发步骤

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

---

## 构建与运行命令（规划中）

> 以下命令尚未验证，因为项目尚未初始化。

- **Rust 侧独立运行**：`cargo run`
- **Tauri 开发模式**：`pnpm tauri dev`（需先初始化 Tauri + 前端）
- **构建发布版**：`pnpm tauri build`

---

## 代码风格与约定

- 项目文档与计划全部使用**中文**撰写，建议后续代码注释也保持一致。
- 前端计划使用 **Vue 3 Composition API + `<script setup>` + TypeScript**。
- Rust 模块按功能分层：`input`（采集）、`engine`（判定）、`db`（持久化）、`notify`（提醒）、`commands`（前后端通信）。

---

## 测试策略

目前未定义自动化测试方案。PLAN.md 中提到的验证方式以**手动测试**为主（运行后观察 console/DB/通知/界面）。建议在后续实现时补充：

- Rust 单元测试（滑动窗口算法、分钟判定逻辑）。
- 前端组件测试（Vue Test Utils / Vitest）。

---

## 安全与隐私

- 全局键鼠监听涉及操作系统底层权限，后续实现需确保：
  - 监听范围仅限于活动计数，不记录按键内容或鼠标轨迹坐标本身。
  - 数据库文件保存在用户本地目录，不向任何网络端点发送数据。
- Tauri 的 `rdev` 与 `active-win-pos-rs` 依赖需要合适的系统权限（macOS 需 Accessibility、Windows 需 UI Access 等）。

---

## 对 AI 助手的提示

1. **不要假设已有代码**：当前仓库没有任何源码，做任何修改前请先初始化对应框架（Tauri / Vue / Vite 等）。
2. **优先参考 PLAN.md**：如果需要在“实现”与“计划”之间做选择，请以 `PLAN.md` 为准，但要在代码注释或文档中标注实际行为是否与计划一致。
3. **保持中文文档**：README、PLAN、AGENTS 均为中文，新增文档建议继续使用中文。
