# 测试策略

## Rust（31 个单元测试）

| 模块 | 数量 | 覆盖 |
|------|------|------|
| `db.rs` | 16 | Block 切分（3）+ 提醒逻辑（11）+ 连续休息（1）+ 喝水记录（1） |
| `reminder.rs` | 4 | snooze / skip / 用户覆盖间隔 / 自动间隔过期 |
| `report.rs` | 4 | versionCode / target 映射 / 签名格式 / 签名规则一致性 |
| `water.rs` | 3 | snooze / 去重 / 喝水后清除 snooze |
| `media_audio.rs` | 4 | 排除列表过滤 / 全排除 / 无音频 / 文本解析 |

## 前端

无自动化测试，依赖手动验证。

## 运行

```bash
cd src-tauri && cargo test
```
