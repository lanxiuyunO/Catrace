# 启动事件上报

## 涉及文件

- `src-tauri/src/report.rs` — 上报逻辑

## 行为

应用启动时（setup 阶段）异步上报 `app_start` 到 `https://api.upgrade.toolsetlink.com/v1/app/report`。

请求头：`X-Timestamp` / `X-Nonce` / `X-AccessKey` / `X-Signature`。

签名规则：`MD5(body=${body}&nonce=${X-Nonce}&secretKey=${SecretKey}&timestamp=${X-Timestamp}&url=/v1/app/report)`。

上报内容：`versionCode`（`major*10000+minor*100+patch`）、`target`（macos→darwin 映射）、`arch`、`devKey`（`dev_${UUID}`，首次生成后持久化）。

上报失败不影响主流程。

## 测试

4 个测试：versionCode / target 映射 / 签名格式 / 签名规则一致性。
